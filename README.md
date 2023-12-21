# LVC camera overlays
Kort gezegd: deze tool kan objecten projecteren op beeld van een PTZ. De PTZs
kunnen via het FreeD protocol[^freed] informatie over rotatie, zoom, en focus
doorgeven aan bijvoorbeeld dit programma. Die informatie kunnen we gebruiken om
objecten, zoals een virtuele finishlijn, te 'projecteren' op het beeld: dit komt
in deze implementatie op een aparte NDI stream die je als overlay kan gebruiken
in vMix of studio monitor.

## Getting started
Installeer als eerste de NDI SDK op je computer (of de computer waar je het
wilt draaien). Je kan [hier](https://ndi.video/download-ndi-sdk/) de link
aanvragen, maar waarschijnlijk staat die ook wel in de LVC-mailbox.

Verder heb je libclang nodig, je hoeft niet te weten wat het is en je hebt het
ook alleen nodig om te compilen.
- Windows: `winget install LLVM.LLVM`
- MacOS: `brew install llvm`
- Linux: installeer afhankelijk van je distro llvm, clang, libclang, [etc.](https://rust-lang.github.io/rust-bindgen/requirements.html)

Als belangrijkste onderdeel heb je Rust op je computer nodig; zie https://rustup.rs/

Zet vervolgende PTZ aan en ga naar de webinterface → CamControl → FreeD. Zet
het aan, vul je IP-adres in en zet bij de port 555{ptz nummer}, bijvoorbeeld
port 5551. Vergeet niet op apply te klikken.

Daarna doe je dit:
```shell
cargo run --release
```

Als het goed is, wordt er nu geluisterd naar de data die de PTZ stuurt en worden
op basis daarvan twee lijnen geprojecteerd. Als je rechtdoor kijkt (0°, 0°),
staat er een plus in het midden van de stream. Deze stream kan je als overlay
gebruiken of gewoon los bekijken, maar daar heb je niet zoveel aan.

[^freed]: zie [doc/FREED.md](doc/FREED.md) voor de essentie van het protocol, uit
[free-d Installation Manual](doc/free-d%20Installation%20Manual%20v1.4.4.pdf)
