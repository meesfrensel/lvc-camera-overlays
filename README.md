# LVC camera overlays
Kort gezegd: deze tool kan objecten projecteren op beeld van een PTZ. De PTZs
kunnen via het FreeD protocol[^freed] informatie over rotatie, zoom, en focus
doorgeven aan bijvoorbeeld dit programma. Die informatie kunnen we gebruiken om
objecten, zoals een virtuele finishlijn, te 'projecteren' op het beeld: dit komt
in deze implementatie op een aparte NDI stream die je als overlay kan gebruiken
in VMix of studio monitor.

## Getting started
Installeer als eerste de NDI SDK op je computer (of de computer waar je het
wilt draaien). Je kan [hier](https://ndi.video/download-ndi-sdk/) de link
aanvragen, maar waarschijnlijk staat die ook wel in de LVC-mailbox.

Zet vervolgende PTZ aan en ga naar de web interface → CamControl → FreeD. Zet
het aan, vul je IP-adres in en zet bij de port 5555. Vergeet niet op apply te
klikken.

Daarna doe je dit (er vanuit gaande dat je Rust op je computer hebt):
```shell
cargo run --release
```

Als het goed is, wordt er nu geluisterd naar de data die de PTZ stuurt en worden
op basis daarvan twee lijnen geprojecteerd. Als je rechtdoor kijkt (0°, 0°),
staat er een plus in het midden van de stream. Deze stream kan je als overlay
gebruiken of gewoon los bekijken, maar daar heb je niet zoveel aan.

[^freed]: zie [doc/FREED.md](doc/FREED.md) voor de essentie van het protocol, uit
[free-d Installation Manual](doc/free-d%20Installation%20Manual%20v1.4.4.pdf)
