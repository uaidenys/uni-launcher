<p align="center">
  <img src="assets/app-icon/hicolor/128x128/apps/ai.storyteller.artcraft-launcher.png" alt="Uni Launcher" width="96">
</p>

<h1 align="center">Uni Launcher</h1>

<p align="center">
  Instala, abre e atualiza os apps Uni da <b>Uniphar Indústria Farmacêutica</b>.
</p>

<p align="center">
  <a href="https://github.com/uaidenys/uni-launcher/releases/latest"><b>Baixar a versão mais recente</b></a>
</p>

---

## Os apps Uni

| App | Para que serve |
|---|---|
| **UniTexto** | Editor de textos: abre e salva documentos do Word |
| **UniPDF** | Leitor e editor de PDF, com assinatura digital (inclusive no padrão ICP-Brasil) |
| **UniPlanner** | Planilhas: abre e salva arquivos do Excel |

O Uni Launcher mostra os apps numa lista, instala o que você escolher e avisa quando sai uma
versão nova. As instalações e atualizações vêm direto das páginas de versões dos apps no GitHub.

## Como instalar

1. Abra a [página da versão mais recente](https://github.com/uaidenys/uni-launcher/releases/latest).
2. Em **Assets**, baixe o arquivo que termina em `-windows-x64.msi`.
3. Abra o arquivo e siga o assistente.
4. Se o Windows mostrar "O Windows protegeu o computador", clique em **Mais informações ›
   Executar assim mesmo**. O instalador ainda não tem assinatura digital de código.

Também há a versão portátil (`-portable.zip`), que roda sem instalar: é só descompactar e abrir.

Requisitos: Windows 10 ou 11, 64 bits.

## Para desenvolvedores

O Uni Launcher é feito em Rust, com [egui](https://github.com/emilk/egui).

```bash
cargo run --release -p artcraft-launcher
```

Para gerar o instalador do Windows (precisa do [WiX v5](https://wixtoolset.org)):

```powershell
powershell -ExecutionPolicy Bypass -File packaging/windows/package.ps1 -Arch x64
```

O arquivo sai em `dist/release/`. Os nomes internos `artcraft-launcher` foram mantidos do projeto
original, para não quebrar a atualização automática.

## Licença e créditos

- **Código:** MIT ou Apache-2.0, à sua escolha ([LICENSE-MIT](LICENSE-MIT),
  [LICENSE-APACHE](LICENSE-APACHE)). Avisos obrigatórios em [NOTICE](NOTICE).
- **Marca:** os nomes Uni e Uniphar, os ícones e os logotipos pertencem à Uniphar Indústria
  Farmacêutica e **não** estão cobertos pela licença do código. Termos em
  [docs/brand/LICENSE-brand.txt](docs/brand/LICENSE-brand.txt). Quem fizer uma versão modificada
  precisa usar nome e identidade visual próprios.
- **Materiais de terceiros:** fontes e ícones com suas licenças, listados em
  [ATTRIBUTION.md](ATTRIBUTION.md).

O Uni Launcher é uma versão modificada do [ArtCraft Launcher](https://github.com/storytold/craft-launcher),
criado pela equipe ArtCraft e colaboradores (MIT OR Apache-2.0).

Desenvolvido por Denys Rodrigues de Oliveira para a Uniphar Indústria Farmacêutica ·
[uniphar.com.br](https://uniphar.com.br/)
