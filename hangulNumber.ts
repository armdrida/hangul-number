export class HangulNumberConverter {
    // Selected 128 readable Hangul syllables (optimized for natural Korean pronunciation)
    private static readonly CHARS = [
        // ㄱ (9): 받침 없거나 부드러운 받침
        '가', '간', '강', '개', '거', '고', '공', '구', '금',
        // ㄴ (9): 자연스러운 발음
        '나', '날', '남', '내', '너', '노', '눈', '늘', '니',
        // ㄷ (9): 일상 단어 느낌
        '다', '달', '담', '대', '더', '도', '동', '두', '드',
        // ㄹ (9): 부드러운 발음
        '라', '람', '랑', '래', '러', '로', '루', '리', '림',
        // ㅁ (9): 자연스러운 조합
        '마', '만', '말', '매', '머', '모', '무', '문', '미',
        // ㅂ (9): 일상적인 글자
        '바', '반', '방', '배', '보', '봄', '부', '비', '빈',
        // ㅅ (10): 많이 쓰이는 글자
        '사', '산', '상', '새', '서', '선', '소', '송', '수', '시',
        // ㅇ (10): 모음 시작 글자
        '아', '안', '양', '어', '연', '영', '오', '온', '우', '이',
        // ㅈ (9): 자연스러운 발음
        '자', '잔', '장', '재', '저', '조', '주', '중', '지',
        // ㅊ (9): 깔끔한 발음
        '차', '찬', '창', '채', '천', '초', '춘', '충', '치',
        // ㅋ (9): 외래어 느낌 최소화
        '카', '칸', '코', '쿠', '크', '키', '캐', '케', '콩',
        // ㅌ (9): 부드러운 발음
        '타', '탄', '태', '터', '토', '통', '투', '트', '티',
        // ㅍ (9): 자연스러운 글자
        '파', '판', '패', '포', '풍', '프', '피', '팔', '품',
        // ㅎ (9): 일상적인 글자
        '하', '한', '해', '허', '호', '홍', '화', '후', '히'
    ];

    // Helper map for O(1) decoding
    private static readonly REVERSE_MAP: Record<string, number> = {};

    static {
        if (this.CHARS.length !== 128) {
            console.warn(`Warning: CHARS size is ${this.CHARS.length}, expected 128.`);
        }
        this.CHARS.forEach((char, index) => {
            this.REVERSE_MAP[char] = index;
        });
    }

    /**
     * Encodes a non-negative integer into a variable-length Hangul string with a specific seed.
     * @param num The number to encode (must be >= 0).
     * @param seed The seed to use for scrambling (0~127).
     * @returns The encoded Hangul string (seed + scrambled data).
     */
    public static encodeWithSeed(num: number, seed: number): string {
        if (num < 0 || !Number.isInteger(num)) {
            throw new Error("Number must be a non-negative integer");
        }
        if (seed < 0 || seed >= 128) {
            throw new Error("Seed must be between 0 and 127");
        }

        const seedChar = this.CHARS[seed];

        // Special case for 0
        if (num === 0) {
            const scrambled = (0 + seed) % 128;
            return seedChar + this.CHARS[scrambled];
        }

        // Convert to base-128 digits (least significant first)
        const digits: number[] = [];
        let temp = num;
        while (temp > 0) {
            digits.push(temp % 128);
            temp = Math.floor(temp / 128);
        }

        // Reverse to get most significant first
        digits.reverse();

        // Scramble each digit with seed and map to Hangul
        const dataChars = digits.map(d => {
            const scrambled = (d + seed) % 128;
            return this.CHARS[scrambled];
        }).join('');

        return seedChar + dataChars;
    }

    /**
     * Encodes a non-negative integer into a variable-length Hangul string.
     * The first character is a random seed that scrambles the remaining characters.
     * Same number can produce different encodings, but all decode to the same value.
     * @param num The number to encode (must be >= 0).
     * @returns The encoded Hangul string (seed + scrambled data).
     */
    public static encode(num: number): string {
        // Random seed (0~127)
        const seed = Math.floor(Math.random() * 128);
        return this.encodeWithSeed(num, seed);
    }

    /**
     * Returns all 128 possible encodings for a number.
     * @param num The number to encode.
     * @returns Array of all 128 encoded strings.
     */
    public static encodeAll(num: number): string[] {
        const results: string[] = [];
        for (let seed = 0; seed < 128; seed++) {
            results.push(this.encodeWithSeed(num, seed));
        }
        return results;
    }

    /**
     * Decodes a Hangul string back to a number.
     * @param str The Hangul string to decode (first char is seed).
     * @returns The decoded number.
     */
    public static decode(str: string): number {
        if (str.length < 2) {
            throw new Error("Invalid string: must be at least 2 characters");
        }

        // First character is the seed
        const seedChar = str[0];
        const seed = this.REVERSE_MAP[seedChar];
        if (seed === undefined) {
            throw new Error(`Invalid seed character: ${seedChar}`);
        }

        // Decode remaining characters
        let num = 0;
        for (let i = 1; i < str.length; i++) {
            const char = str[i];
            const scrambled = this.REVERSE_MAP[char];
            if (scrambled === undefined) {
                throw new Error(`Invalid character: ${char}`);
            }
            // Unscramble: original = (scrambled - seed + 128) % 128
            const original = (scrambled - seed + 128) % 128;
            num = num * 128 + original;
        }

        return num;
    }
}



// Interactive CLI
import * as readline from 'readline';
import { fileURLToPath } from 'url';

// Simple check to see if this file is being run primarily
// equivalent to if (import.meta.url === pathToFileURL(process.argv[1]).href)
// But for simplicity in ts-node/demos, we can just run it.
// If we want to be safe for importing later:
// if (process.argv[1] === fileURLToPath(import.meta.url)) { ... }

if (true) {
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout
    });

    console.log("=== Hangul Number Converter (Base-128, Variable Length) ===");
    console.log("Enter a non-negative integer to encode.");
    console.log("Type 'exit' to quit.\n");

    const prompt = () => {
        rl.question('Enter number: ', (answer) => {
            if (answer.toLowerCase() === 'exit') {
                rl.close();
                return;
            }

            const num = parseInt(answer.replace(/,/g, ''), 10);

            if (isNaN(num)) {
                console.log("Please enter a valid number.\n");
                prompt();
                return;
            }

            if (num < 0) {
                console.log("Number must be non-negative.\n");
                prompt();
                return;
            }

            console.log(`\nAll 128 encodings for ${num.toLocaleString()}:`);
            console.log("--------------------------------------------------");

            try {
                const allEncodings = HangulNumberConverter.encodeAll(num);
                
                // Print in rows of 8 for readability
                for (let row = 0; row < 16; row++) {
                    const items: string[] = [];
                    for (let col = 0; col < 8; col++) {
                        const idx = row * 8 + col;
                        const encoded = allEncodings[idx];
                        const decoded = HangulNumberConverter.decode(encoded);
                        const check = decoded === num ? "✓" : "✗";
                        items.push(`${encoded}${check}`);
                    }
                    console.log(items.join('  '));
                }
                
                console.log(`\nTotal: 128 variants, Length: ${allEncodings[0].length} chars each`);
            } catch (e) {
                console.error(`Error: ${e}`);
            }
            console.log("--------------------------------------------------\n");
            prompt();
        });
    };

    prompt();
}
