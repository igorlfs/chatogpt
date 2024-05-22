# Trabalho Prático - Teste de Software

## Grupo

- Luís Felipe Ramos Ferreira
- Igor Lacerda Faria da Silva
- João Vitor Mateus Silva
- Gabriela Assunção Fonseca

## Explicação do Sistema

O _ChatGPT_ é um grande modelo de linguagem desenvolvido pela empresa _OpenAI_ disponibilizado ao público de forma gratuita em algumas de suas versões. Ele pode ser útil em um vasto conjunto de tarefas de processamento de linguagem natural, como por exemplo correção de error ortográficos, respostas de perguntas, traduções de textos, redigir emails e outros tipos de textos e até mesmo criar suas pŕoprias histórias.

Ele se tornou muito popular devido à sua capacidade de gerar textos rápidos e de alta qualidade que podem ajudar os usuários a resolver diversos tipos de tarefas.

Nesse contexto, dada a importância e popularidade do _chat_, nosso grupo se propôs a criar uma aplicação similiar, mas em contexto mais descontraído. O _ChatoGPT_ é uma sátira da ferramenta previamente citada, onde a ideia principal não é ser útil e sim gerar entretenimento ao tentar desenvolver uma conversa com a aplicação. Portanto, Enquanto o _ChatGPT_ foca em fornecer respostas precisas e úteis, o _ChatoGPT_ tem o objetivo de fazer as pessoas rirem e se divertirem com respostas inusitadas e humorísticas. Abaixo podemos citar algumas de suas principais _features_:

### _Features_ do aplicativo _ChatoGPT_

1. Respostas engraçadas e sem sentido

O _ChatoGPT_ fornece respostas que, baseadas ou não na pergunta inicial, são exageradas, sarcásticas ou simplesmente hilárias. Por exemplo, se um usuário pergunta sobre a previsão do tempo, o _ChatoGPT_ poderia responder algo como: "Vai chover sapos e arco-íris".

Um outro usuário poderia perguntar também algo como: "Que horas vai ser o show do MC hat?" e o _ChatoGPT_ responder como: "As 18h e ônibus". (Para quem tiver curiosidade sobre a piada: <https://www.youtube.com/watch?v=VRRGIjsJNLo>).

2. Sarcasmo

Algumas respostas do _ChatoGPT_ são carregadas de sarcasmo, tentando tirar sarro da pergunta ou dele mesmo. Por exemplo, se alguém pergunta como fazer uma receita de bolo de chocolate, o **ChatoGPT** pode responder: "Que preguiça de pesquisar no Google hein! Mas vamos lá: primeiro você precisar de chocolate ...".

3. Mensagens secretas

O _ChatoGPT_ pode pregar algumas peças as vezes também. Sua resposta pode até vir correta e com um conteúdo sério e profissional, mas ele vai te sacanear e enviar uma mensagem criptografada! Um usuário pode enviar a pergunta "Quem é maior, o Cruzeiro ou o Atlético?" e a resposta vai ser "ervtpwjc!". (Nesse caso todo mundo sabe que a resposta é Cruzeiro, mas pode nem sempre ser tão fácil...)

Mas qual tipo de cifra ele usa? Bom, boa sorte pra descobrir.

4. Texto confuso

O _ChatoGPT_ também pode responder corretamente sua pergunta, mas vai querer fazer uma piada e vai brincar com o texto da resposta. Um exemplo é quando ele alterna entre letras minúsculas e maiúsculas o texto da resposta. Por exemplo, o usuário pode perguntar "Qual o verdadeiro nome do Batman?" e a resposta vai ser "O vErDaDeIrO nOmE dO BaTmAn É cLaRk KeNt!"

Essas são só algumas das _features_ do _ChatoGPT_. Mais serão adicionados para que ele seja uma ferramenta completa para você se divertir.

## Tecnologias Utilizadas

O projeto foi implementado com o **Tauri**, que se trata de um **framework** que permite a criação de aplicativos de **desktop** usando tecnologias como HTML, CSS e JavaScript. Ela funciona com um front-end em JavaScript e um back-end em Rust. O Tauri é uma alternativa ao Electron, mas é mais seguro, rápido e compacto. Os testes foram implementados usando as funcionalidades nativas de Rust. O framework usado no front foi o React.

Para implementar algo que se parecesse ao menos com um modelo e linguagem, utilizamos a API Rest gratuita disponibilizada pelo modelo Gemini da Google. Desse modo, podemos enviar as perguntas que os usuários mandam para o _ChatoGPT_ diretamente para uma requisição da API e manipulamos a resposta da maneira que for melhor. Em alguns casos, utilizamos _prompts_ alternativos que garantem que a resposta enviada pelo próprio Gemini já seja satírica e e graçada. Em outros, apenas pegamos a resposta e manipulamos com nosso próprio código, como nos casos em que criptografamos o conteúdo do texto ou alternamos entre minúsculas e maiúsculas o conteúdo da resposta.
