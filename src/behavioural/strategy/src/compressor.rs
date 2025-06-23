/// 压缩策略接口
pub trait CompressorStrategy {
    fn compress(&self, input: &str) -> String;
    fn decompress(&self, input: &str) -> String;
}

/// 无压缩策略
pub struct NoCompression;

impl CompressorStrategy for NoCompression {
    fn compress(&self, input: &str) -> String {
        input.to_string()
    }

    fn decompress(&self, input: &str) -> String {
        input.to_string()
    }
}

/// 简单压缩策略：aaabbc -> a3b2c1
pub struct SimpleCompression;

impl CompressorStrategy for SimpleCompression {
    fn compress(&self, input: &str) -> String {
        let mut compressed = String::new();
        let mut chars = input.chars();
        let Some(mut current_char) = chars.next() else { return compressed; };
        let mut count = 1;

        for c in chars {
            if c == current_char {
                count += 1;
            } else {
                compressed.push(current_char);
                compressed.push_str(&count.to_string());
                current_char = c;
                count = 1;
            }
        }

        compressed.push(current_char);
        compressed.push_str(&count.to_string());

        compressed
    }

    fn decompress(&self, input: &str) -> String {
        let mut decompressed = String::new();
        let mut chars = input.chars();
        let mut peeked: Option<char> = None;

        while let Some(c) = peeked.take().or_else(|| chars.next()) {
            let mut count_str = String::new();
            while let Some(digit) = chars.next() {
                if digit.is_digit(10) {
                    count_str.push(digit);
                } else {
                    peeked = Some(digit);
                    break;
                }
            }
            let count = count_str.parse::<usize>().unwrap();
            decompressed.push_str(&c.to_string().repeat(count));
        }

        decompressed
    }
}

/// Compressor 上下文，持有策略 trait object
pub struct Compressor {
    strategy: Box<dyn CompressorStrategy>,
}

impl Compressor {
    /// 创建 Compressor，传入策略对象
    pub fn new(strategy: Box<dyn CompressorStrategy>) -> Self {
        Self { strategy }
    }

    /// 切换策略
    pub fn set_strategy(&mut self, strategy: Box<dyn CompressorStrategy>) {
        self.strategy = strategy;
    }

    /// 压缩
    pub fn compress(&self, input: &str) -> String {
        self.strategy.compress(input)
    }

    /// 解压缩
    pub fn decompress(&self, input: &str) -> String {
        self.strategy.decompress(input)
    }
}
