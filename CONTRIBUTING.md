# Contribuindo com o Dusty

Obrigado por querer contribuir com o Dusty! Aqui estão algumas diretrizes para ajudar você a começar.


---

## Filosofia do Projeto

O Dusty é mantido por um grupo de desenvolvimento, que atualmente só participa eu, o criador do Dusty.

Aceitamos: 

- Código produzido por qualquer pessoa
- Issues, ideias, forks, etc, sem problemas

O que nao aceitamos:

- A unica coisa que nao aceito é push direto ao GitLab / GitHub

---

## Entao, como contribuir?

Crie um fork do repositório do GitLab, contribua com o que desejar, e envie um Merge Reqwest

## Pré-requisitos

Antes de tudo, certifique-se de ter conhecimento, mesmo que basico em:

- Rust
- Linux
- Projetos Open-Source

---

## Como configurar o ambiente

```shell
git clone https://gitlab.com/pedrotbhc/Dusty.git dusty/
cd dusty
cargo run --release
```

---

## Onde contribuir

### Áreas onde ajuda é bem-vinda:

- UI: melhorias na Ratatui, navegação, cores, animações
- Aprimoramento / desenvolvimento de novos sistemas: é essencial para manter o Dusty sempre funcionando bem
- Performance: multiplas otimizações podem see feitas no ecossistema do Dusty
- Design: temas, visual, personalização 
- Internacionalização: ajuda na tradução de texto para o Dusty
- Marketing: como o Dusty é exibido na internet

---

## Estrutura básica do projeto

- GitLab: no GitLab é onde acontece as atualizações em tempo real, todas as alterações que ocorrem no Dusty, primeiro chegam no GitLab
- GitHub: versões estáveis do Dusty

---

## Estilo de código e boas práticas

- Use clippy antes de subir qualquer coisa
- Sempre coloque comentários em formato de documentação para códigos fora do comum
- Prefira match ao invés de unwrap(), exceto onde faça sentido
- Nome de funções e variáveis em inglês, mesmo que o texto esteja traduzido

---

## Formas de Contribuir

#### Mesmo sem push, você pode contribuir com:
- Abrindo uma Issue com:
- Sugestões de funcionalidades
- Relatos de bugs
- Ideias para otimização

---

## O que evitar

- Subir código quebrado
- Incluir dependências muito pesadas sem discussão
- Ignorar a arquitetura atual do projeto
- Misturar muitos assuntos num só PR

---

## Dúvidas?

Fale comigo: [pedrotbhc](https://gitlab.com/pedrotbhc)
Ou abra uma Issue no GitHub com a tag question.

