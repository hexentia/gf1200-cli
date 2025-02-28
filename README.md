# gf1200-cli
Um CLI pro roteador [GF-1200](https://www.intelbras.com/pt-br/roteador-wi-fi-5-ac-1200-com-porta-internet-giga-e-lan-fast-wi-force-gf-1200) da Intelbras.

## Instalação
Esse projeto é feito pra Linux, e pode ser instalado usando
[Nix](https://nixos.org/).

<details>
<summary>sem Nix</summary>

Com [Rust](http://rust-lang.org/), `pkg-config` e `openssl` instalados,

1. Clone o projeto.
2. Na pasta do projeto, `cargo install --path .` pra instalar no perfil atual.

...et voilà.
</details>

<details>
<summary>com Nix/no NixOS</summary>

Esse projeto é um flake, podendo ser consumido de várias formas (rodado sem instalar com `nix run`,
instalado impuramente com `nix install`, etc.).
O pacote é exposto como `packages.${system}.default`.

Por exemplo, pra instalar declarativamente num NixOS (`x86_64-linux` ou `aarch64`):
```nix
# flake.nix
{
    inputs.gf1200-cli.url = "github:hexentia/gf1200-cli";
    # ...
}
```

```nix
# configuration.nix (ou outro arquivo)
{ inputs, system, ... }: {
    environment.systemPackages = [
        inputs.gf1200-cli.packages.${system}.default
    ];
    # ...
}
```
</details>


## Uso
`gf1200-cli` depois de instalado pra rodar. 
A interface funciona como um [REPL](https://pt.wikipedia.org/wiki/REPL). O comando `help`
lista todos os comandos disponível. Pra sair, `exit` ou Ctrl-C (ou Ctrl-D).

Se uma variável `GF1200_ADDR` existir, ela vai ser usada como IP do roteador direto.

## Progresso
#### Essencial
- [x] Funcionalidades básicas (status, restart)
- [ ] Configuração (de LAN, WAN e sistema)

#### Extra
- [ ] Auto-completion
- [ ] Chamada única (pra uso com pipes `|`)

## Licença
Unlicense (i.e. domínio público).
