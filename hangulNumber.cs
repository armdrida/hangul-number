using System;
using System.Collections.Generic;
using System.Text;

public class HangulNumberConverter
{
    // Selected 128 readable Hangul syllables (optimized for natural Korean pronunciation)
    private static readonly string[] CHARS = new string[]
    {
        // ㄱ (9): 받침 없거나 부드러운 받침
        "가", "간", "강", "개", "거", "고", "공", "구", "금",
        // ㄴ (9): 자연스러운 발음
        "나", "날", "남", "내", "너", "노", "눈", "늘", "니",
        // ㄷ (9): 일상 단어 느낌
        "다", "달", "담", "대", "더", "도", "동", "두", "드",
        // ㄹ (9): 부드러운 발음
        "라", "람", "랑", "래", "러", "로", "루", "리", "림",
        // ㅁ (9): 자연스러운 조합
        "마", "만", "말", "매", "머", "모", "무", "문", "미",
        // ㅂ (9): 일상적인 글자
        "바", "반", "방", "배", "보", "봄", "부", "비", "빈",
        // ㅅ (10): 많이 쓰이는 글자
        "사", "산", "상", "새", "서", "선", "소", "송", "수", "시",
        // ㅇ (10): 모음 시작 글자
        "아", "안", "양", "어", "연", "영", "오", "온", "우", "이",
        // ㅈ (9): 자연스러운 발음
        "자", "잔", "장", "재", "저", "조", "주", "중", "지",
        // ㅊ (9): 깔끔한 발음
        "차", "찬", "창", "채", "천", "초", "춘", "충", "치",
        // ㅋ (9): 외래어 느낌 최소화
        "카", "칸", "코", "쿠", "크", "키", "캐", "케", "콩",
        // ㅌ (9): 부드러운 발음
        "타", "탄", "태", "터", "토", "통", "투", "트", "티",
        // ㅍ (9): 자연스러운 글자
        "파", "판", "패", "포", "풍", "프", "피", "팔", "품",
        // ㅎ (9): 일상적인 글자
        "하", "한", "해", "허", "호", "홍", "화", "후", "히"
    };

    // Helper map for O(1) decoding
    private static readonly Dictionary<string, int> REVERSE_MAP = new Dictionary<string, int>();

    private static readonly Random _random = new Random();

    static HangulNumberConverter()
    {
        if (CHARS.Length != 128)
        {
            Console.WriteLine($"Warning: CHARS size is {CHARS.Length}, expected 128.");
        }
        for (int i = 0; i < CHARS.Length; i++)
        {
            REVERSE_MAP[CHARS[i]] = i;
        }
    }

    /// <summary>
    /// Encodes a non-negative integer into a variable-length Hangul string with a specific seed.
    /// </summary>
    /// <param name="num">The number to encode (must be >= 0).</param>
    /// <param name="seed">The seed to use for scrambling (0~127).</param>
    /// <returns>The encoded Hangul string (seed + scrambled data).</returns>
    public static string EncodeWithSeed(long num, int seed)
    {
        if (num < 0)
        {
            throw new ArgumentException("Number must be a non-negative integer");
        }
        if (seed < 0 || seed >= 128)
        {
            throw new ArgumentException("Seed must be between 0 and 127");
        }

        string seedChar = CHARS[seed];

        // Special case for 0
        if (num == 0)
        {
            int scrambled = (0 + seed) % 128;
            return seedChar + CHARS[scrambled];
        }

        // Convert to base-128 digits (least significant first)
        var digits = new List<int>();
        long temp = num;
        while (temp > 0)
        {
            digits.Add((int)(temp % 128));
            temp /= 128;
        }

        // Reverse to get most significant first
        digits.Reverse();

        // Scramble each digit with seed and map to Hangul
        var sb = new StringBuilder();
        sb.Append(seedChar);
        foreach (int d in digits)
        {
            int scrambled = (d + seed) % 128;
            sb.Append(CHARS[scrambled]);
        }

        return sb.ToString();
    }

    /// <summary>
    /// Encodes a non-negative integer into a variable-length Hangul string.
    /// The first character is a random seed that scrambles the remaining characters.
    /// Same number can produce different encodings, but all decode to the same value.
    /// </summary>
    /// <param name="num">The number to encode (must be >= 0).</param>
    /// <returns>The encoded Hangul string (seed + scrambled data).</returns>
    public static string Encode(long num)
    {
        // Random seed (0~127)
        int seed = _random.Next(128);
        return EncodeWithSeed(num, seed);
    }

    /// <summary>
    /// Returns all 128 possible encodings for a number.
    /// </summary>
    /// <param name="num">The number to encode.</param>
    /// <returns>Array of all 128 encoded strings.</returns>
    public static string[] EncodeAll(long num)
    {
        var results = new string[128];
        for (int seed = 0; seed < 128; seed++)
        {
            results[seed] = EncodeWithSeed(num, seed);
        }
        return results;
    }

    /// <summary>
    /// Decodes a Hangul string back to a number.
    /// </summary>
    /// <param name="str">The Hangul string to decode (first char is seed).</param>
    /// <returns>The decoded number.</returns>
    public static long Decode(string str)
    {
        if (string.IsNullOrEmpty(str))
        {
            throw new ArgumentException("Invalid string: must not be null or empty");
        }

        // Get grapheme clusters (each Hangul character is one grapheme)
        var chars = GetGraphemeClusters(str);
        
        if (chars.Count < 2)
        {
            throw new ArgumentException("Invalid string: must be at least 2 characters");
        }

        // First character is the seed
        string seedChar = chars[0];
        if (!REVERSE_MAP.TryGetValue(seedChar, out int seed))
        {
            throw new ArgumentException($"Invalid seed character: {seedChar}");
        }

        // Decode remaining characters
        long num = 0;
        for (int i = 1; i < chars.Count; i++)
        {
            string c = chars[i];
            if (!REVERSE_MAP.TryGetValue(c, out int scrambled))
            {
                throw new ArgumentException($"Invalid character: {c}");
            }
            // Unscramble: original = (scrambled - seed + 128) % 128
            int original = (scrambled - seed + 128) % 128;
            num = num * 128 + original;
        }

        return num;
    }

    /// <summary>
    /// Splits a string into grapheme clusters (handles multi-byte Unicode characters).
    /// </summary>
    private static List<string> GetGraphemeClusters(string str)
    {
        var result = new List<string>();
        var enumerator = System.Globalization.StringInfo.GetTextElementEnumerator(str);
        while (enumerator.MoveNext())
        {
            result.Add(enumerator.GetTextElement());
        }
        return result;
    }
}

// Interactive CLI
public class Program
{
    public static void Main(string[] args)
    {
        Console.OutputEncoding = System.Text.Encoding.UTF8;
        
        Console.WriteLine("=== Hangul Number Converter (Base-128, Variable Length) ===");
        Console.WriteLine("Enter a non-negative integer to encode.");
        Console.WriteLine("Type 'exit' to quit.\n");

        while (true)
        {
            Console.Write("Enter number: ");
            string? answer = Console.ReadLine();

            if (string.IsNullOrEmpty(answer) || answer.ToLower() == "exit")
            {
                break;
            }

            // Remove commas and parse
            string cleanedAnswer = answer.Replace(",", "");
            if (!long.TryParse(cleanedAnswer, out long num))
            {
                Console.WriteLine("Please enter a valid number.\n");
                continue;
            }

            if (num < 0)
            {
                Console.WriteLine("Number must be non-negative.\n");
                continue;
            }

            Console.WriteLine($"\nAll 128 encodings for {num:N0}:");
            Console.WriteLine("--------------------------------------------------");

            try
            {
                string[] allEncodings = HangulNumberConverter.EncodeAll(num);

                // Print in rows of 8 for readability
                for (int row = 0; row < 16; row++)
                {
                    var items = new List<string>();
                    for (int col = 0; col < 8; col++)
                    {
                        int idx = row * 8 + col;
                        string encoded = allEncodings[idx];
                        long decoded = HangulNumberConverter.Decode(encoded);
                        string check = decoded == num ? "✓" : "✗";
                        items.Add($"{encoded}{check}");
                    }
                    Console.WriteLine(string.Join("  ", items));
                }

                Console.WriteLine($"\nTotal: 128 variants, Length: {allEncodings[0].Length} chars each");
            }
            catch (Exception e)
            {
                Console.WriteLine($"Error: {e.Message}");
            }
            Console.WriteLine("--------------------------------------------------\n");
        }
    }
}
