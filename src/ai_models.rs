use crate::i18n::Language;
use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIModel {
    DeepSeek,
    Qwen,
    ChatGPT,
    Claude,
    Gemini,
    Llama,
    Custom,
}

impl AIModel {
    pub fn all() -> Vec<Self> {
        vec![
            Self::DeepSeek,
            Self::Qwen,
            Self::ChatGPT,
            Self::Claude,
            Self::Gemini,
            Self::Llama,
            Self::Custom,
            Self::Custom,
            Self::Custom,
            Self::Custom,
            Self::Custom,
            Self::Custom,
            Self::Custom,
            Self::Custom,
            Self::Custom,
            Self::Custom,
            Self::Custom,
        ]
    }

    pub fn name(&self, language: Language) -> String {
        match language {
            Language::Chinese => match self {
                Self::DeepSeek => "DeepSeek".to_string(),
                Self::ChatGPT => "ChatGPT".to_string(),
                Self::Claude => "Claude".to_string(),
                Self::Gemini => "Gemini".to_string(),
                Self::Llama => "Llama".to_string(),
                Self::Qwen => "通义千问".to_string(),
                Self::Custom => "自定义模型".to_string(),
            },
            Language::English => match self {
                Self::DeepSeek => "DeepSeek".to_string(),
                Self::ChatGPT => "ChatGPT".to_string(),
                Self::Claude => "Claude".to_string(),
                Self::Gemini => "Gemini".to_string(),
                Self::Llama => "Llama".to_string(),
                Self::Qwen => "Qwen".to_string(),
                Self::Custom => "Custom".to_string(),
            },
        }
    }

    pub fn colors(&self) -> (Color, Color, Color) {
        match self {
            Self::DeepSeek => (
                Color::Rgb(0, 150, 255),
                Color::Rgb(0, 100, 200),
                Color::Rgb(200, 230, 255),
            ),
            Self::ChatGPT => (
                Color::Rgb(16, 163, 127),
                Color::Rgb(12, 122, 95),
                Color::Rgb(220, 255, 240),
            ),
            Self::Claude => (
                Color::Rgb(147, 51, 234),
                Color::Rgb(126, 34, 206),
                Color::Rgb(240, 220, 255),
            ),
            Self::Gemini => (
                Color::Rgb(251, 188, 4),
                Color::Rgb(234, 88, 12),
                Color::Rgb(255, 248, 220),
            ),
            Self::Llama => (
                Color::Rgb(220, 38, 38),
                Color::Rgb(185, 28, 28),
                Color::Rgb(255, 230, 230),
            ),
            Self::Qwen => (
                Color::Rgb(59, 130, 246),
                Color::Rgb(37, 99, 235),
                Color::Rgb(219, 234, 254),
            ),
            Self::Custom => (
                Color::Rgb(139, 92, 246),
                Color::Rgb(124, 58, 237),
                Color::Rgb(233, 213, 255),
            ),
        }
    }

    pub fn style(&self, is_selected: bool) -> Style {
        let (primary, secondary, text) = self.colors();
        if is_selected {
            Style::default()
                .fg(text)
                .bg(primary)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(primary).add_modifier(Modifier::BOLD)
        }
    }

    pub fn simulate_response(&self, user_input: &str, language: Language) -> String {
        match language {
            Language::Chinese => self.simulate_response_chinese(user_input),
            Language::English => self.simulate_response_english(user_input),
        }
    }

    fn simulate_response_chinese(&self, user_input: &str) -> String {
        match self {
            Self::DeepSeek => format!(
                "我是 DeepSeek，深度求索开发的 AI 助手！您的问题：\"{}\"\n\n\
                让我为您提供详细的解答...\n\
                • 分析问题核心\n\
                • 提供实用建议\n\
                • 推荐学习资源\n\n\
                如果您有更多疑问，随时告诉我！",
                user_input
            ),
            Self::ChatGPT => format!(
                "我是 ChatGPT，OpenAI 的 AI 助手！关于：\"{}\"\n\n\
                让我为您分析：\n\
                • 技术实现方案\n\
                • 创意灵感\n\
                • 学习路径建议\n\n\
                有什么具体需求吗？",
                user_input
            ),
            Self::Claude => format!(
                "我是 Claude，Anthropic 的 AI 助手！您提到：\"{}\"\n\n\
                我的分析：\n\
                • 问题结构化分解\n\
                • 风险评估\n\
                • 系统化思考建议\n\n\
                希望这对您有帮助！",
                user_input
            ),
            Self::Gemini => format!(
                "我是 Gemini，Google 的 AI 助手！您的问题：\"{}\"\n\n\
                我的回答：\n\
                • 最新趋势分析\n\
                • 实用操作建议\n\
                • 学习资源推荐\n\n\
                需要更多信息吗？",
                user_input
            ),
            Self::Llama => format!(
                "我是 Llama，Meta 的开源 AI 模型！关于：\"{}\"\n\n\
                开源视角：\n\
                • 社区解决方案\n\
                • 技术架构建议\n\
                • 协作开发指导\n\n\
                欢迎交流讨论！",
                user_input
            ),
            Self::Qwen => format!(
                "我是通义千问，阿里云的 AI 助手！您的问题：\"{}\"\n\n\
                中文场景建议：\n\
                • 语言文化适配\n\
                • 本地化策略\n\
                • 中国市场洞察\n\n\
                需要进一步帮助吗？",
                user_input
            ),
            Self::Custom => format!(
                "我是自定义 AI 模型！您的问题：\"{}\"\n\n\
                可配置响应：\n\
                • 响应风格调整\n\
                • 专业领域聚焦\n\
                • 详细程度控制\n\n\
                告诉我您的偏好！",
                user_input
            ),
        }
    }

    fn simulate_response_english(&self, user_input: &str) -> String {
        match self {
            Self::DeepSeek => format!(
                "I'm DeepSeek, an AI assistant from DeepSeek Company! Your question: \"{}\"\n\n\
                Let me provide a detailed answer...\n\
                • Analyze core issues\n\
                • Provide practical suggestions\n\
                • Recommend learning resources\n\n\
                Feel free to ask more!",
                user_input
            ),
            Self::ChatGPT => format!(
                "I'm ChatGPT, an AI assistant from OpenAI! About: \"{}\"\n\n\
                My analysis:\n\
                • Technical implementation\n\
                • Creative inspiration\n\
                • Learning path suggestions\n\n\
                Any specific needs?",
                user_input
            ),
            Self::Claude => format!(
                "I'm Claude, an AI assistant from Anthropic! You mentioned: \"{}\"\n\n\
                My analysis:\n\
                • Problem structuring\n\
                • Risk assessment\n\
                • Systematic thinking\n\n\
                Hope this helps!",
                user_input
            ),
            Self::Gemini => format!(
                "I'm Gemini, an AI assistant from Google! Your question: \"{}\"\n\n\
                My response:\n\
                • Latest trends analysis\n\
                • Practical suggestions\n\
                • Learning resources\n\n\
                Need more information?",
                user_input
            ),
            Self::Llama => format!(
                "I'm Llama, an open-source AI model from Meta! About: \"{}\"\n\n\
                Open-source perspective:\n\
                • Community solutions\n\
                • Technical architecture\n\
                • Collaboration guidance\n\n\
                Welcome to discuss!",
                user_input
            ),
            Self::Qwen => format!(
                "I'm Qwen, an AI assistant from Alibaba Cloud! Your question: \"{}\"\n\n\
                Chinese context suggestions:\n\
                • Language adaptation\n\
                • Localization strategy\n\
                • China market insights\n\n\
                Need further help?",
                user_input
            ),
            Self::Custom => format!(
                "I'm a Custom AI model! Your question: \"{}\"\n\n\
                Configurable responses:\n\
                • Response style adjustment\n\
                • Professional focus\n\
                • Detail level control\n\n\
                Tell me your preferences!",
                user_input
            ),
        }
    }
}
