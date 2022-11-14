<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->

<!-- PROJECT LOGO -->
<br />
<div align="center">

<h3 align="center">Kniffel in Rust</h3>

  <p align="center">
    Ein privates Project, dass ich erstellt habe um Rust zu lernen.
    <br />
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#about-the-project">About The Project</a></li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#installation">Installation</a></li>
        <li><a href="#building">Building</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

Ich wollte nur Rust lernen, also hab ich eine quicke Runde Kiffel für zwei Spieler erstellt.

<p align="right">(<a href="#readme-top">back to top</a>)</p>




<!-- GETTING STARTED -->
## Getting Started


### Installation

* Lade den neuesten Release runter
* Öffne PowerShell oder CMD im Ordner in der sich die kniffel.exe befindet und führe `.\Kniffel.exe` aus

### Building

* Installiere Rustup von [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
*  ```sh
   git clone https://github.com/FabiSahne/kniffel
   cd kniffel
   cargo build --release
   ```
* kniffel.exe befindet sich in /target/release/

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

Gib in der Konsole (im Ordner wo sich die exe befindet) `.\kniffel.exe` ein, das Programm startet.

Spieler 1 ist an der Reihe und es wurde bereits gewürfelt.
Entweder trägst du dein Ergebnis direkt ins Scoreboard, oder du würfelst erneut.
Wähle mit 1, oder 2 aus, und bestätige mit Enter. Falls du nochmal würfelst, hast du noch die möglichkeit Würfel zu behalten.
Diese Würfel werden auf die Seite gelegt und nicht mitgewürfelt.
Wähle aus indem du eingibst welche Würfel du gerne behalten würdest
(z.B: `123` wenn du die linken 3 Würfel behalten willst, mit den rechten beiden würde nun weiter gewürfelt werden).
Du kannst 3 mal Würfeln bevor du eintragen musst.

Nun ist Spieler 2 an der Reihe.

<p align="right">(<a href="#readme-top">back to top</a>)</p>




<!-- CONTRIBUTING -->
## Contributing

Auch wenn das Open Source ist, ist es dennoch ein privates Projekt von mir.
Wenn ihr wollt, gerne Forken, aber ich werde keine Pullrequests annehmen.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the Unlicense license. See `UNLICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Fabian Wolter - fabianwolter@outlook.com

Project Link: [https://github.com/FabiSahne/kniffel](https://github.com/FabiSahne/kniffel)

<p align="right">(<a href="#readme-top">back to top</a>)</p>
