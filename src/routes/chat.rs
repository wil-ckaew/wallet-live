// src/routes/chat.rs
use askama::Template;
use axum::{
    Router, routing::{get, post},
    response::{Html, IntoResponse, Json},
    extract::Path,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Local;
use rand::Rng;

use crate::{
    app::AppState,
    error::AppError,
    auth::user::User,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/chat", get(chat_page))
        .route("/api/chat", post(chat_message))
        .route("/api/chat/history/{user_id}", get(chat_history))
}

#[derive(Template)]
#[template(path = "chat.html")]
pub struct ChatPage {
    pub user: User,
}

async fn chat_page(user: User) -> Result<Html<String>, AppError> {
    let page = ChatPage { user };
    Ok(Html(page.render()?))
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub message: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatHistory {
    pub user_id: i64,
    pub messages: Vec<ChatMessage>,
}

use lazy_static::lazy_static;

lazy_static! {
    static ref CHAT_HISTORY: Arc<Mutex<Vec<ChatHistory>>> = Arc::new(Mutex::new(Vec::new()));
}

async fn chat_message(
    user: User,
    Json(request): Json<ChatRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user_message = request.message.clone();
    let ai_response = generate_ai_response(&request.message);
    let timestamp = Local::now().to_rfc3339();
    
    let mut history = CHAT_HISTORY.lock().await;
    if let Some(user_history) = history.iter_mut().find(|h| h.user_id == user.id()) {
        user_history.messages.push(ChatMessage {
            role: "user".to_string(),
            content: user_message.clone(),
            timestamp: timestamp.clone(),
        });
        user_history.messages.push(ChatMessage {
            role: "assistant".to_string(),
            content: ai_response.clone(),
            timestamp: Local::now().to_rfc3339(),
        });
    } else {
        history.push(ChatHistory {
            user_id: user.id(),
            messages: vec![
                ChatMessage {
                    role: "user".to_string(),
                    content: user_message,
                    timestamp: timestamp.clone(),
                },
                ChatMessage {
                    role: "assistant".to_string(),
                    content: ai_response.clone(),
                    timestamp: Local::now().to_rfc3339(),
                },
            ],
        });
    }
    
    Ok(Json(ChatResponse {
        message: ai_response,
        timestamp: Local::now().to_rfc3339(),
    }))
}

async fn chat_history(
    user: User,
    Path(user_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let history = CHAT_HISTORY.lock().await;
    let user_history = history.iter().find(|h| h.user_id == user_id);
    
    match user_history {
        Some(h) => Ok(Json(h.clone())),
        None => Ok(Json(ChatHistory {
            user_id,
            messages: vec![],
        })),
    }
}

fn generate_ai_response(message: &str) -> String {
    let mut rng = rand::thread_rng();
    let message_lower = message.to_lowercase();
    
    // SAUDAÇÕES
    if message_lower.contains("oi") || message_lower.contains("olá") || message_lower.contains("ola") {
        let responses = vec![
            "Olá! 👋 Como posso ajudar você com seus investimentos hoje?",
            "Oi! 😊 Estou aqui para ajudar com suas dúvidas sobre o mercado financeiro!",
            "Olá! Que bom ter você aqui. Vamos conversar sobre investimentos? 💰",
        ];
        return responses[rng.gen_range(0..responses.len())].to_string();
    }
    
    // BITCOIN
    if message_lower.contains("bitcoin") || message_lower.contains("btc") {
        let responses = vec![
            "📊 **Bitcoin (BTC)** está cotado a **$67,432.50**. Tendência de alta com suporte em $65,000.",
            "🚀 Bitcoin cresceu **12.4%** nos últimos 30 dias. Ótimo momento para investir!",
            "💎 Dominância do Bitcoin está em **52.3%**. Continua sendo a criptomoeda mais sólida.",
        ];
        return responses[rng.gen_range(0..responses.len())].to_string();
    }
    
    // ETHEREUM
    if message_lower.contains("ethereum") || message_lower.contains("eth") {
        let responses = vec![
            "🔷 **Ethereum (ETH)** está em **$3,456.78**. Alta atividade em DeFi e NFTs.",
            "📈 Ethereum subiu **8.2%** na última semana. Interesse institucional crescendo.",
            "⚡ Upgrade do Ethereum trouxe mais eficiência e escalabilidade para a rede.",
        ];
        return responses[rng.gen_range(0..responses.len())].to_string();
    }
    
    // QUAL MOEDA INVESTIR
    if message_lower.contains("qual moeda") || 
       (message_lower.contains("moeda") && message_lower.contains("investir")) ||
       message_lower.contains("qual cripto") {
        let responses = vec![
            "💰 **Recomendação para iniciantes:**\n\n• **Bitcoin (70%)** - Segurança e estabilidade\n• **Ethereum (20%)** - Potencial de crescimento\n• **Stablecoins (10%)** - USDC ou USDT\n\n📈 Comece com esses e diversifique gradualmente!",
            
            "📊 **Top 5 criptomoedas:**\n\n1. **Bitcoin (BTC)** - Ouro digital\n2. **Ethereum (ETH)** - Contratos inteligentes\n3. **BNB** - Exchange líder\n4. **Solana (SOL)** - Escalabilidade\n5. **XRP** - Pagamentos internacionais",
        ];
        return responses[rng.gen_range(0..responses.len())].to_string();
    }
    
    // MELHOR INVESTIMENTO
    if message_lower.contains("melhor investimento") || 
       message_lower.contains("melhor investir") ||
       message_lower.contains("onde investir") {
        let responses = vec![
            "💎 **Melhores investimentos em 2024:**\n\n1. **Bitcoin** - Maior rentabilidade\n2. **Ethereum** - Inovação e adoção\n3. **Ações tecnologia** - Apple, Microsoft\n4. **Fundos imobiliários** - Renda passiva\n5. **Tesouro Direto** - Segurança",
            
            "📊 **Ranking de rentabilidade:**\n\n🥇 **Bitcoin** +156%\n🥈 **Ethereum** +98%\n🥉 **S&P500** +28%\n4️⃣ **Ouro** +15%\n5️⃣ **Poupança** +6%",
        ];
        return responses[rng.gen_range(0..responses.len())].to_string();
    }
    
    // EXPLIQUE SOBRE INVESTIMENTO
    if message_lower.contains("explique sobre investimento") ||
       message_lower.contains("o que é investimento") ||
       message_lower.contains("como funciona investimento") {
        let responses = vec![
            "📚 **O que é Investimento?**\n\nInvestir é alocar dinheiro em um ativo esperando retorno futuro.\n\n**Tipos:**\n1. **Renda Fixa** - Baixo risco\n2. **Renda Variável** - Médio/alto risco\n3. **Fundos** - Diversificação\n4. **Criptomoedas** - Ativos digitais\n\n🎯 Diversifique e tenha disciplina!",
            
            "💡 **Guia para iniciantes:**\n\n1️⃣ **Educação** - Estude antes\n2️⃣ **Objetivos** - Defina metas\n3️⃣ **Planejamento** - Crie estratégia\n4️⃣ **Diversificação** - Não coloque tudo em um lugar\n5️⃣ **Paciência** - Resultados vêm com o tempo",
        ];
        return responses[rng.gen_range(0..responses.len())].to_string();
    }
    
    // COMO COMEÇAR
    if message_lower.contains("como começar") || 
       message_lower.contains("começar a investir") ||
       message_lower.contains("primeiros passos") {
        let responses = vec![
            "🚀 **Como começar a investir:**\n\n1️⃣ **Educação** - Leia livros, faça cursos\n2️⃣ **Defina objetivos** - Curto, médio e longo prazo\n3️⃣ **Escolha uma corretora** - Binance, Coinbase\n4️⃣ **Comece pequeno** - Valores baixos\n5️⃣ **Acompanhe** - Monitore regularmente\n\n📚 Livros: 'Pai Rico, Pai Pobre' e 'O Investidor Inteligente'",
            
            "🎯 **Roteiro para iniciantes:**\n\n**Mês 1-3:** Estude os fundamentos\n**Mês 4-6:** Abra conta em corretora\n**Mês 7-9:** Faça primeiros investimentos\n**Mês 10-12:** Aprenda com erros e ajuste",
        ];
        return responses[rng.gen_range(0..responses.len())].to_string();
    }
    
    // DICAS DE SEGURANÇA
    if message_lower.contains("segurança") || 
       message_lower.contains("seguro") ||
       message_lower.contains("proteger") {
        let responses = vec![
            "🛡️ **Dicas de segurança:**\n\n1. Use **autenticação 2FA**\n2. Nunca compartilhe **chaves privadas**\n3. Desconfie de **promessas de lucro fácil**\n4. Verifique a **reputação** da corretora\n5. Use **carteiras frias** para grandes quantias\n\n🔐 Sua segurança é prioridade!",
            
            "⚠️ **Sinais de alerta:**\n\n❌ Promessas de retorno garantido\n❌ Pressão para investir rápido\n❌ Contato não solicitado\n❌ Sites sem informações\n❌ Sem registro na CVM",
        ];
        return responses[rng.gen_range(0..responses.len())].to_string();
    }
    
    // TENDÊNCIAS 2024
    if message_lower.contains("tendencias 2024") || 
       message_lower.contains("tendências 2024") ||
       message_lower.contains("futuro") ||
       message_lower.contains("previsão") {
        let responses = vec![
            "🔮 **Tendências para 2024:**\n\n1. **Bitcoin** - Previsão de US$ 100.000+\n2. **Ethereum** - Upgrade e escalabilidade\n3. **IA e Blockchain** - Integração crescente\n4. **Tokenização** - Ativos reais em blockchain\n5. **Regulamentação** - Maior clareza jurídica\n\n📈 Futuro promissor para criptomoedas!",
        ];
        return responses[rng.gen_range(0..responses.len())].to_string();
    }
    
    // MERCADO
    if message_lower.contains("mercado") || 
       message_lower.contains("tendencia") ||
       message_lower.contains("tendência") {
        let responses = vec![
            "📊 Mercado em **alta consolidada**. Expectativa positiva para os próximos meses.",
            "🚀 Adoção institucional crescendo. Tendência de valorização no longo prazo.",
            "📈 Mercado apresenta volatilidade saudável. Ótimo para oportunidades de entrada.",
        ];
        return responses[rng.gen_range(0..responses.len())].to_string();
    }
    
    // CARTEIRA
    if message_lower.contains("carteira") || 
       message_lower.contains("portfolio") ||
       message_lower.contains("portfólio") {
        let responses = vec![
            "📋 Sua carteira está bem diversificada. Monitore a performance diária.",
            "📅 Revise sua carteira **mensalmente** para ajustar investimentos.",
            "📈 Lembre-se do **longo prazo**. Volatilidade é normal no mercado.",
        ];
        return responses[rng.gen_range(0..responses.len())].to_string();
    }
    
    // AJUDA
    if message_lower.contains("ajuda") || message_lower.contains("help") {
        return "🤖 **Posso ajudar você com:**\n\n• 📊 **Bitcoin** - Preço e análises\n• 🔷 **Ethereum** - Preço e tendências\n• 💰 **Investimentos** - Dicas e estratégias\n• 📋 **Carteira** - Gestão de portfólio\n• 📈 **Mercado** - Tendências e análises\n• 🛡️ **Segurança** - Dicas de proteção\n• 🚀 **Como começar** - Primeiros passos\n• 💎 **Qual moeda investir** - Recomendações\n\nPergunte-me qualquer coisa! 🚀".to_string();
    }
    
    // RESPOSTA PADRÃO
    let responses = vec![
        "🤔 Entendi! Como posso ajudar você com seus investimentos hoje?",
        "📊 Interessante! Posso te dar mais informações sobre isso.",
        "💡 Ótima pergunta! Vou analisar isso para você.",
        "🤖 Fico feliz em ajudar com suas dúvidas sobre investimentos.",
    ];
    responses[rng.gen_range(0..responses.len())].to_string()
}