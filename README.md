# 💰 Wallet Live

Uma aplicação web moderna para gerenciamento de portfólio de ativos financeiros, construída com Rust e tecnologia de ponta.

![Wallet Live Dashboard](screenshots/dashboard.png)

## 📸 Telas do Sistema

### 1. Tela de Login
![Login](screenshots/login.png)
*Tela de autenticação com design elegante e campos intuitivos*

### 2. Dashboard Principal
![Dashboard](screenshots/dashboard.png)
*Visão geral com métricas e gráfico de performance*

### 3. Gerenciamento de Ativos
![Assets](screenshots/assets.png)
*Lista completa de ativos com ações de gerenciamento*

## 🚀 Demonstração

O projeto inclui:

- **Dashboard interativo** com gráficos de performance
- **Gerenciamento completo de ativos** (CRUD)
- **Autenticação segura** com JWT
- **Histórico de compras** detalhado
- **Design moderno** com efeito glass morphism

## 🛠️ Tecnologias Utilizadas

### Backend
| Tecnologia | Descrição |
|------------|-----------|
| **Rust** | Linguagem principal |
| **Axum** | Framework web |
| **SQLx** | ORM assíncrono |
| **PostgreSQL** | Banco de dados |
| **JWT** | Autenticação |
| **Askama** | Template engine |

### Frontend
| Tecnologia | Descrição |
|------------|-----------|
| **Tailwind CSS** | Estilização |
| **Plotly** | Gráficos interativos |
| **HTML5/CSS3** | Estrutura e estilos |

### Ferramentas
| Ferramenta | Descrição |
|------------|-----------|
| **Docker** | Containerização |
| **Cargo** | Gerenciador de pacotes |
| **Git** | Controle de versão |

## 📋 Funcionalidades

### 🔐 Autenticação
- ✅ Login seguro com JWT
- ✅ Registro automático de novos usuários
- ✅ Cookies HTTP-only para segurança
- ✅ Sessão persistente

### 📊 Dashboard
- ✅ Métricas em tempo real:
  - Valor total do portfólio
  - Total investido
  - Lucro/Prejuízo
  - Quantidade de ativos
- ✅ Gráfico de performance (30 dias)
- ✅ Top performers (5 melhores ativos)

### 💼 Gerenciamento de Ativos
- ✅ **Visualização**: Lista completa com cards expansíveis
- ✅ **Criação**: Adicionar novos ativos
- ✅ **Edição**: Atualizar nome e valor unitário
- ✅ **Exclusão**: Remover ativos com confirmação
- ✅ **Compra**: Registrar compras de ativos
- ✅ **Histórico**: Visualizar todas as compras

### 🎨 Design
- ✅ Interface moderna com glass morphism
- ✅ Tema escuro para melhor experiência
- ✅ Animações suaves e responsivas
- ✅ Indicadores visuais de lucro/prejuízo (verde/vermelho)
- ✅ Ícones intuitivos para cada funcionalidade

## 🏗️ Estrutura do Projeto
wallet-live/
├── src/
│ ├── app.rs # Configuração da aplicação
│ ├── auth/ # Autenticação
│ │ └── user.rs # Gerenciamento de usuários
│ ├── error.rs # Tratamento de erros
│ ├── main.rs # Ponto de entrada
│ ├── models.rs # Modelos de dados
│ ├── repository.rs # Camada de banco de dados
│ └── routes/ # Rotas da aplicação
│ ├── api.rs # API REST
│ ├── dashboard.rs # Dashboard
│ └── frontend.rs # Interface web
├── templates/ # Templates HTML
│ ├── login.html
│ ├── assets.html
│ ├── dashboard.html
│ ├── new_asset.html
│ ├── edit_asset.html
│ ├── asset_detail.html
│ └── asset_detail_dashboard.html
├── migrations/ # Migrações SQL
├── fixtures/ # Dados de teste
├── screenshots/ # Capturas de tela
│ ├── login.png
│ ├── dashboard.png
│ └── assets.png
├── docker-compose.yml # Configuração Docker
├── Cargo.toml # Dependências
└── .env # Variáveis de ambiente
text


## 🚦 Pré-requisitos

- **Rust** (última versão estável)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    PostgreSQL (via Docker ou local)
    bash

    # Docker
    docker run -d --name postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres:latest

    # Ubuntu
    sudo apt install postgresql postgresql-contrib

    Docker e Docker Compose (opcional)
    bash

    sudo apt install docker.io docker-compose

    Cargo (gerenciador de pacotes - já vem com Rust)

📦 Instalação
1. Clone o repositório
bash

git clone https://github.com/seu-usuario/wallet-live.git
cd wallet-live

2. Configure o banco de dados
bash

# Inicie o PostgreSQL com Docker
docker-compose up -d db

# Ou use o comando direto
docker run -d --name postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres:latest

3. Configure as variáveis de ambiente

Crie um arquivo .env na raiz do projeto:
env

DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres?sslmode=disable

4. Execute as migrações
bash

# Instale o SQLx CLI se ainda não tiver
cargo install sqlx-cli

# Execute as migrações
cargo sqlx migrate run

5. Instale as dependências
bash

cargo build

6. Execute a aplicação
bash

cargo run

Acesse: http://localhost:3000
🗄️ Estrutura do Banco de Dados
Tabelas
Users
sql

CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

Assets
sql

CREATE TABLE assets (
    id BIGSERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    unit_value DOUBLE PRECISION NOT NULL
);

Owned Assets
sql

CREATE TABLE owned_assets (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    asset_id BIGINT REFERENCES assets(id) ON DELETE CASCADE,
    quantity_owned DOUBLE PRECISION NOT NULL,
    bought_for DOUBLE PRECISION NOT NULL,
    bought_at TIMESTAMP DEFAULT NOW(),
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_owned_assets_user_id ON owned_assets(user_id);
CREATE INDEX idx_owned_assets_asset_id ON owned_assets(asset_id);

🧪 Testes
bash

# Executar todos os testes
cargo test

# Executar testes específicos
cargo test test_create_asset

# Executar com saída detalhada
cargo test -- --nocapture

📱 Endpoints
API REST
Método	Endpoint	Descrição
GET	/api/assets	Listar todos os ativos
POST	/api/assets	Criar novo ativo
PATCH	/api/assets	Atualizar ativo
Interface Web
Rota	Descrição
/	Página inicial / Dashboard
/login	Tela de login
/logout	Logout
/assets	Lista de ativos
/assets/new	Criar novo ativo
/assets/{id}	Detalhes do ativo
/assets/{id}/edit	Editar ativo
/dashboard	Dashboard com gráficos
🐳 Docker
Iniciar apenas o banco de dados
bash

docker-compose up -d db

Iniciar tudo (aplicação + banco)
bash

docker-compose up -d

Ver logs
bash

docker-compose logs -f

Parar os containers
bash

docker-compose down

Remover volumes
bash

docker-compose down -v

🔒 Segurança

    Senhas: Hash com password-auth (Argon2)

    Tokens: JWT com expiração de 10 minutos

    Cookies: HttpOnly e Secure

    SQL: Prepared statements com SQLx

    CORS: Configuração adequada para produção

🎨 Personalização
Cores

O tema usa paleta cyber/tech:
Cor	Valor	Uso
Primária	#22d3ee (Cyan)	Botões principais, links
Secundária	#3b82f6 (Blue)	Gradientes
Sucesso	#34d399 (Emerald)	Lucros, botões de ação
Erro	#f87171 (Red)	Prejuízos, exclusão
Fundo	#0f172a a #1e1b4b	Gradiente de fundo
Fontes

    Inter - Para textos gerais

    Space Mono - Para elementos de código

🛠️ Desenvolvimento
Comandos úteis
bash

# Compilar em modo desenvolvimento
cargo build

# Compilar em modo release (otimizado)
cargo build --release

# Executar
cargo run

# Verificar erros sem compilar
cargo check

# Formatar código
cargo fmt

# Verificar com linter
cargo clippy

# Atualizar dependências
cargo update

# Verificar dependências desatualizadas
cargo outdated

Comandos SQLx
bash

# Criar nova migration
cargo sqlx migrate add --timestamp nome_da_migration

# Executar migrations
cargo sqlx migrate run

# Reverter última migration
cargo sqlx migrate revert

# Verificar status das migrations
cargo sqlx migrate info

🤝 Contribuição

    Fork o projeto

    Crie sua branch (git checkout -b feature/nova-feature)

    Commit suas mudanças (git commit -m 'Adiciona nova feature')

    Push para a branch (git push origin feature/nova-feature)

    Abra um Pull Request

Padrões de código

    Use cargo fmt para formatação

    Use cargo clippy para linting

    Escreva testes para novas funcionalidades

    Mantenha a documentação atualizada

📝 Licença

Este projeto está sob a licença MIT. Veja o arquivo LICENSE para mais detalhes.
🙏 Agradecimentos

    Rust - Linguagem incrível

    Axum - Framework web

    Tailwind CSS - Estilização

    Plotly - Gráficos interativos

    SQLx - ORM assíncrono

📧 Contato

    Autor: wilson Archimedes da Silva

    Email: wil.kimel@gmail.com
    GitHub: github.com/wil-ckaew
    LinkedIn: linkedin.com/in/wilson-archimedes-da-silva-90968948/
⭐ Suporte

📚 O que Aprendi

Durante este desafio, tive a oportunidade de:

    Domínio de Rust: Aprimorei meu conhecimento em Rust, especialmente com:

        Tokio para programação assíncrona

        SQLx para operações seguras com banco de dados

        Axum para construção de APIs robustas

        Askama para templates HTML

    Arquitetura de Software: Estruturei uma aplicação Fullstack com:

        Separação clara de responsabilidades

        Organização de código modular

        Princípios SOLID aplicados

    Segurança: Implementei medidas de segurança importantes:

        JWT para autenticação stateless

        Hash de senhas com Argon2

        Proteção contra SQL Injection

        Cookies seguros

    UX/UI: Desenvolvi uma interface moderna com:

        Glass Morphism

        Tema escuro

        Animações suaves

        Gráficos interativos

    DevOps: Utilizei:

        Docker para containerização

        SQLx CLI para migrações

        Git para versionamento

🏆 Resultado

O projeto final é uma aplicação funcional, segura e com uma interface moderna que permite:

    ✅ Autenticação de usuários

    ✅ Gerenciamento completo de ativos

    ✅ Visualização de portfólio

    ✅ Dashboard com gráficos

    ✅ Histórico de compras

    ✅ Design responsivo

🚀 Próximos Passos

    Integrar com APIs de mercado (Binance, CoinGecko)

    Adicionar alertas de preço

    Exportar relatórios (PDF/CSV)

    Autenticação 2FA

    Mobile App (React Native)

    Análise de portfólio avançada

📝 Licença

Este projeto está sob a licença MIT.
🙏 Agradecimentos

    DIO (Digital Innovation One) pelo desafio

    Comunidade Rust pelo suporte

    Todos os contribuidores do projeto

Desafio concluído com sucesso! 