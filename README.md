<a name="readme-top"></a>

[![GPL-3 License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]


<!-- PROJECT LOGO -->
<div align="center">
  <h1 align="center">SEND-TO-BIN</h1>

  <p align="center">
    A mush safer alternative for the linux 'rm' command built with Rust
    <br />
    <a href="https://github.com/dhairyagupta2603/Send-To-Bin-CLI"><strong>Explore the docs »</strong></a>
    <br />
  </p>
</div>


<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#license">License</a></li>
    
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

This project aims to be a drop-in replacement for the 'rm' command for basic use cases and scenarios. This has some similar functionality to windows recycle bin. This is the first project that I have build in Rust as a exercise to learn and understand its fundamental syntax and package management. 

Regardless, here are some features that make it usefull: -
* It doesn't require any additional configurations for most popular distributions (checked in debian)
* It stores all the "deleted" files in under a folder in home directory until you clear the bin. clearing is also a single intuitive command. 
* Biggest improvemt is the UNDO command which restores the previously deleted files, very useful when you delete your entire project :sunglasses:

<!-- GETTING STARTED -->
## Getting Started
### Prerequisites

* The system should have a ```.bashrc``` file in the ```$HOME``` directory so that project path and bin path can be specified when installing in linux. 

* The system should also have ```/usr/local/bin``` to write the binary into. If you want to customize the path, you can change the ```$dest``` variable it in the ```linux-install.sh``` file.

### Installation

Installing this project is a very simple process. You just need the following steps: -
1. Clone the repo anyware on your system 
    ```sh
    git clone "https://github.com/dhairyagupta2603/Send-To-Bin-CLI.git"
    ```

2. Run the ```linux-install.sh``` in the root of the project directory
   ```sh
   cd Send-To-Bin-CLI

   sh linux-install.sh
   # or
   bash linux-install.sh 
   # or 
   zsh linux-install.sh
   ```
3. There is no 3rd step. You are done! :star: 

<!-- USAGE EXAMPLES -->
## Usage

The commands are as follows: -
* Initializing the project is done by 
  ```sh
  stb init
  ```
* 'Deleting' files is as simple as 
  ```sh
  stb file1.txt ../file2.json src/rsc/ 
  ```
* If you want to undo the previous command just do 
  ```sh
  stb undo
  ```
* If you want to clear the bin, you guessed it just
  ```sh
  stb clear
  # or to skip confirmation
  stb clear -y
  ```
* If you don't want to use stb anymore, you can also do 
  ```sh
  stb destroy
  # or if bin is not empty
  stb destroy -f 
  ```
  then remove the projectb path form your ```.bashrc```

<!-- LICENSE -->
## License

Distributed under the GPL-3 License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
[license-shield]: https://img.shields.io/badge/LICENSE-GPL-3?style=for-the-badge
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/dhairya-gupta-2603m/
[license-url]: https://github.com/dhairyagupta2603/Send-To-Bin-CLI/blob/main/LICENSE.txt
