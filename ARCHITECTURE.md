# 🏗 Arquitetura do Shred Extension Rs (C-FFI & Rust) 🦀

Criar uma extensão para os ecossistemas do GNOME (Nautilus usando GTK4/Libadwaita API) e XFCE (Thunar) não é uma tarefa simples com as transições das antigas *crates* do Rust. Este documento cobre como as barreiras do C-FFI foram superadas e as soluções criativas projetadas na extensão para suportar ambos os gerenciadores de arquivos simultaneamente com uma base de código unificada.

## 1. O Abandono de Crates Rust Defasados e a adoção do "Pure FFI"

Originalmente, existiam tentativas com *crates* da comunidade, como `thunar-extension` ou antigos binding do Nautilus. No entanto, esses pacotes muitas vezes estavam presos a versões defasadas de pacotes C (ex: `gtk-sys v0.15` amarrado ao GTK3).

Nos níveis mais novos do GTK4 (Nautilus/GNOME 43+) e nas atualizações do Thunar, estruturas clássicas da base C foram alteradas ou banidas da biblioteca de extensões padrão. Um simples link via *crate* antigo gerava uma chuva de *"Undefined Symbols"* na hora que a extensão carregava.

**A Solução Multi-Ambiente:** Descartamos completamente os *crates* de ligação obsoletos. Construímos as **próprias ligações C-FFI (Rust FFI)** da extensão, apontando dependências diretamente à memória nativa do sistema do File Manager correspondente (`libnautilus-extension` e `libthunarx`).
Através de anotações como `#[cfg(feature = "nautilus")]` e `#[cfg(feature = "thunar")]`, o Rust consegue expor a VTable exata implementando os ponteiros corretos. No lugar de uma pesada dependência ao ecossistema GTK completo do Rust, usamos apenas conversão enxuta garantida pelo `gio` nativo. O provedor de menus então é injetado diretamente usando `g_type_module_register_type`.

## 2. Bloqueio Assíncrono do Nautilus / Thunar (UI Freezing)

Nas primeiras abordagens da lógica do *shred*, notamos que se colocássemos _loops_ iterativos sobrescrevendo arquivos de megabytes dentro do _callback_ do menu de contexto, todo o Gerenciador de Arquivos entrava em _Freeze_ (A *main thread* congelava até a exclusão terminar). 

**A Solução de Assincronicidade:**
Implementamos uma simples rotina de concorrência com `std::thread::spawn` atrelando o processamento agressivo da deleção a **Threads de Fundo Distintas**. Isso permite que o callback que acionamos a partir do GLib/GTK retorne rapidamente à memória, fazendo a experiência visual ao clicar reaparecer na tela quase que antes de você notar; enquanto os discos continuam rodando o `shred` furtivamente no vazio em plano de fundo.

## 3. O Defeito Visual Nativo de `shred -u`

Usar a flag clássica `-u` obriga o executável nativo `shred` (do shell linux) a ofuscar caminhos no momento da obliteração. Ele renomeia o arquivo intensivamente contendo caracteres com zeros e sequências variadas (ex: "000000", "00").
Embora isso seja essencial, deixava "lixo temporário" visível no mesmo nível hierárquico da pasta por diversos _nano-segundos_. Como os UI Views do Nautilus/Thunar são dinâmicos e extremamente rápidos, eles muitas vezes capturavam e renderizavam o F5 visual destes arquivos falsos temporários e deturpavam a vista do próprio usuário durante esta etapa.

**A Solução da Camuflagem:**
Desenvolvemos uma manipulação lógica instantânea. Extraímos o arquivo original e efetuamos um `std::fs::rename` muito rápido em rust movendo o arquivo da vista dos exploradores (modificando os inodes localmente), o encapsulando dentro de **Subdiretórios Secretos Virtuais**.
Tais diretórios subjacentes recebem a assinatura `.~shred_RANDOMSID` atrelada. Por iniciarem com um ponto `.`, o ecossistema Unix os denota como "invisíveis" logo por padrão. Agora, todas as sujeiras do `shred -u` operam livremente na escuridão sem causar problemas de UX. Padrão Absoluto de UX e Transparência.

## 4. Sem Dependências Pesadas de Terceiros e "Dialogs" Nativos

Normalmente, *extensions pipelines* usam Gettext (.mo/.po files), que exigiriam compilação adicional na base dos scripts para os menus e as caixas de alerta. Depender de `gtk4-rs` injetaria centenas de megabytes de dados de ligação estática no tamanho do binário `.so`, só para produzir uma janela pop-up de diálogo.

* **Micromotor interno i18n:** Optamos por criar um _struct_ micro-gerenciador pro *i18n*. Lendo nossa própria variável de ambiente do OS (`$LANG`), instanciamos dados pré-computados localizados em Português, Espanhol ou o Padrão Inglês, sem as bagagens do gettext.
* **Zenity App Callbacks:** Recusamos injetar ligações libadwaita/gtk4 nos _alerts_ em Rust. Em substituição, chamamos um _fork spawn_ `zenity` — a excelente caixa de diálogos C-App que interage com o tema GTK nativo da DE e é pre-instalada garantida na maioria dos GNOME e desktops modernos. Recuperamos então as respotas (*exit codes*) e definimos o percurso destrutivo de acordo se o usuário clicou em OK ou Cancelar. Assim, alcançamos um binário com a extrema leveza e Zero Custos de `cargo build`.
