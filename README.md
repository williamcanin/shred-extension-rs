# Shred Extension Rs

[![Build Status](https://img.shields.io/github/actions/workflow/status/williamcanin/shred-extension-rs/release.yml?logo=github)](https://github.com/williamcanin/shred-extension-rs/actions)
[![Rust](https://img.shields.io/badge/Language-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20GNOME%20%26%20XFCE-blue?logo=linux)](#)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

Uma extensão nativa e ultraleve para os gerenciadores de arquivos **Nautilus (GNOME)** e **Thunar (XFCE)** que adiciona a opção de exclusão permanente com padrão governamental (**Shred**) no menu de contexto dos seus arquivos.

Escrita em **Rust**, a extensão foca na alta performance e máxima segurança ao destruir dados. Ela se comunica diretamente via APIs com o nível base do C (GObject / libnautilus-extension / libthunarx), sem camadas inchadas de dependências desatualizadas.

## ✨ Features

* **Nativo no Menu de Contexto**: Opção explícita "Excluir com Segurança" visível em qualquer arquivo do navegador padrão do GNOME ou XFCE.
* **Interface Assíncrona & Instantânea**: Quando confirmada a exclusão, o arquivo é imediatamente ocultado e some visualmente da interface de forma instantânea. A exclusão de fato ocorre silenciosamente em uma **background thread**, mantendo o explorador arquivos responsivo e não afetando sua navegação.
* **Caixa de Confirmação Integrada**: Para evitar cliques acidentais e dores de cabeça que o comando `shred` pode causar se acionado por engano, a extensão chama a Dialog Box nativa (`zenity`) do sistema. Requer o "Sim/OK" para sumir de vez com o item.
* **Camuflagem Inteligente de Arquivo**: Diferente do comando nativo `shred -u` no terminal (que polui e pisca a sua pasta de arquivos com os nomes criptografados antes de desfazê-los, como `000000`), esta extensão camufla perfeitamente os arquivos através de diretórios temporários "invisíveis" (iniciados com `.`) para o Gerenciador de Arquivos. A deleção acontece por trás de cortinas, silenciosamente.
* **Suporte Multi-Ambiente**: Compatível tanto com o **Nautilus** quanto o **Thunar**, compartilhando a mesma base de código em Rust.
* **Internacionalização (i18n) Embutida**: O software é portátil, e possui detecção automática de idioma baseando-se no SO local para exibir todas as mensagens e menus em:

  * 🇧🇷 Português-BR / PT (*"Excluir com Segurança"*)
  * 🇪🇸 Espanhol (*"Eliminación Segura"*)
  * 🇺🇸 Inglês Padrão / Fallback (*"Secure Delete"*)

---

## 📦 Instalação

Você pode instalar a extensão de três formas diferentes:

1. **Arch Linux (AUR)** — pacotes separados para cada file manager:

   * `shred-extension-rs-nautilus`
   * `shred-extension-rs-thunar`
2. **Script automático universal (qualquer distro)**
3. **Instalação manual da biblioteca**

---

### 🐧 Arch Linux (AUR)

Se você utiliza Arch Linux ou derivadas (EndeavourOS, Manjaro, CachyOS, etc), instale diretamente pelo AUR escolhendo o file manager desejado:

```bash
yay -S shred-extension-rs-nautilus
```

ou

```bash
yay -S shred-extension-rs-thunar
```

Ou manualmente:

```bash
git clone https://aur.archlinux.org/shred-extension-rs-nautilus.git
cd shred-extension-rs-nautilus
makepkg -si
```

ou

```bash
git clone https://aur.archlinux.org/shred-extension-rs-thunar.git
cd shred-extension-rs-thunar
makepkg -si
```

Os pacotes instalam a biblioteca no local correto do sistema usando o nome padrão:

```
libshred_extension_rs.so
```

Totalmente compatível com atualizações futuras e sem conflito com o instalador por script.

---

### 🚀 Instalação Automática (Qualquer distribuição Linux)

Você pode instalar **sem baixar nada**, executando o script diretamente do repositório oficial:

```bash
bash <(curl -fsSL https://raw.githubusercontent.com/williamcanin/shred-extension-rs/main/install.sh)
```

Ou usando `wget`:

```bash
bash <(wget -qO- https://raw.githubusercontent.com/williamcanin/shred-extension-rs/main/install.sh)
```

O script:

* Pergunta qual file manager você deseja (Nautilus ou Thunar)
* Baixa automaticamente a versão mais recente via API do GitHub
* Instala a biblioteca no diretório correto do sistema
* Padroniza o nome para `libshred_extension_rs.so`, garantindo compatibilidade com o AUR

> 💡 Dica de segurança: você pode inspecionar o script antes de executar acessando:
> [https://raw.githubusercontent.com/williamcanin/shred-extension-rs/main/install.sh](https://raw.githubusercontent.com/williamcanin/shred-extension-rs/main/install.sh)

---

### 🧰 Instalação Manual

Se preferir gerenciar manualmente usando comandos `root`, você precisará da biblioteca contida nos nossos Releases (`.so`).

Após baixar o arquivo do Release, renomeie para o padrão esperado:

```bash
mv libshred-extension-rs-*.so libshred_extension_rs.so
```

* **Para Nautilus:**

```bash
sudo cp libshred_extension_rs.so /usr/lib/nautilus/extensions-4/
nautilus -q
```

*(Alguns sistemas como Ubuntu/Mint podem usar `/usr/lib/x86_64-linux-gnu/nautilus/extensions-4/`)*

* **Para Thunar:**

```bash
sudo cp libshred_extension_rs.so /usr/lib/thunarx-3/
thunar -q
```

*(Algumas distribuições usam `/usr/lib/x86_64-linux-gnu/thunarx-3/`)*

---

## 🧑‍💻 Motivação & Aprofundamento Backend (Arquitetura)

Se você tem interesse em Engenharia de Software, integração com C-FFI, e como driblamos as limitações de bibliotecas no Rust que bloqueiam a *main thread* do GTK, confira nosso arquivo [ARCHITECTURE.md](ARCHITECTURE.md) com os detalhes das soluções adotadas.

---

**Aviso de Uso:** *A sobrescrita ocorre em 3 etapas acompanhadas por preenchimento em "0". Esta ferramenta utiliza deleção de alta segurança, sendo os processos aplicados **irreversíveis**. Portanto, certifique-se bem de onde clica!*
