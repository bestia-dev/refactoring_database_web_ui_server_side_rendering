<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

# refactoring_database_web_ui_server_side_rendering

[//]: # (auto_cargo_toml_to_md start)

**08. Tutorial to refactor the database_web_ui_on_server (2022-10)**  
***version: 0.1.01 date: 2022-10-10 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/refactoring_database_web_ui_server_side_rendering)***  

[//]: # (auto_cargo_toml_to_md end)

 ![maintained](https://img.shields.io/badge/maintained-green)
 ![ready_for_use](https://img.shields.io/badge/ready_for_use-green)
 ![tutorial](https://img.shields.io/badge/tutorial-yellow)
 ![youtube](https://img.shields.io/badge/youtube-yellow)

[//]: # (auto_lines_of_code start)

[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-230-green.svg)](https://github.com/bestia-dev/refactoring_database_web_ui_server_side_rendering/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-9-blue.svg)](https://github.com/bestia-dev/refactoring_database_web_ui_server_side_rendering/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-28-purple.svg)](https://github.com/bestia-dev/refactoring_database_web_ui_server_side_rendering/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/refactoring_database_web_ui_server_side_rendering/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/refactoring_database_web_ui_server_side_rendering/)

[//]: # (auto_lines_of_code end)

 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/refactoring_database_web_ui_server_side_rendering/blob/main/LICENSE)
 [![Rust](https://github.com/bestia-dev/refactoring_database_web_ui_server_side_rendering/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/refactoring_database_web_ui_server_side_rendering/)
 ![refactoring_database_web_ui_server_side_rendering](https://bestia.dev/webpage_hit_counter/get_svg_image/627386887.svg)

Hashtags: #rust #rustlang #tutorial  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Intro

In the [last tutorial](https://github.com/bestia-dev/database_web_ui_on_server) we created a working prototype for a web application that can create, read, update and delete (CRUD) data in a postgres database. The user interface is server side rendered as html5 and css3. The code is pretty basic and repetitive, because the main focus was on concepts, tools, libraries and data flow.
We will continue to develop this project in the 8th part of the [Rust tutorial series](https://www.youtube.com/channel/UCitt3zFHK2jDetDh6ezI05A). We will refactor this web app to deduplicate code and make it more idiomatic Rust.  

This project has also a youtube video tutorial. Watch it:
<!-- markdownlint-disable MD033 -->
[<img src="https://bestia.dev/youtube/refactoring_database_web_ui_server_side_rendering.jpg?_" width="400px">](https://bestia.dev/youtube/refactoring_database_web_ui_server_side_rendering.html)
<!-- markdownlint-enable MD033 -->

## Motivation

The first function of software is to solve a problem. In this project, the user can read and write data in a database simply using a browser.  
The second main function of software is maintainability. Software will always with no exception need to be updated or upgraded for any reason. When we write our source code, we want to make it easier and less error-prone to make changes later.  
Rust is the champion of fearless refactoring. The strict type system warns the developer immediately if he forgets something. And I assure you, that happens a lot.  

## Separate code

Every programming language have a way to separate code into logical modules. If that does not exist, we can still use names by prefixing them with a namespace. That will also work. This distinction is just for the human brain to reason about one problem at a time. The computer itself does not need to know about this separation.  
Our project is 3-tier and it is not a single coherent project, not even the same programming language. If we want to understand easily how the data flows from one project to the other, it is smart to give items the same name everywhere. We can then use the simple "Find text" tool of VSCode to find everything about an item.  

## Web app name

The project name is long and descriptive because this is a tutorial. But the executable binary can have a more meaningful name like "webpage_hits_admin".  
It is enough to rename the folder in "src/bin/" to change the name of the binary. I will also rename the web_server_folder and css file accordingly.  
Because of this change, I will change the name inside the automation_task_rs to reflect the new binary name.  

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
