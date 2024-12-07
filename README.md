![wlc_banner](https://github.com/user-attachments/assets/ddcacbde-c3a6-45ab-92c3-aaa7dd2b35de)

# SWLC  
#### Searching the Westminster Leningrad Codex

<br>

# NOTE

This project is under heavy development. Please note that all aspects of this project are subject to change along the road, due evolving requirements and changes in insight.


<br>


# Motivation

Although there are already websites/programs that you can use to search the Westminster Leningrad Codex (the oldest complete version of the Old Testament written in the Hebrew language).  For me personally, however, I have not yet found options that provides sufficient search capabilities. 

While learning Biblical Hebrew and Rust I decided to combine both languanges. So I started to write a search program in Rust for the Westminster Leningrad Codex. The idea is that I write this missing features.


Here are some points I miss most in existing search engines:

1. **MAQAF and word finding**   
   The maqaf is a type of hyphen used to connect two words. In Hebrew, this is not a new word, but it is more of a “sound unit” made up of two separate words. I should be able to find each of those separate words.

2. **Consonants and diacritics**  
   Finding specific combinations of consonants and diacritics is not possible. You usually get a list of the requested consonants including all diacritics. This means you still have to manually search the given list for your specific diacritics.
   
<br>

# Layout

THIS WILL MOST CERTAINLY CHANGE DURING DEVELOPMENT!

The package layout for now will be the following:

**swcl_cli**   
A crate containing the CLI version of the search program.

**swcl_gui**  
A crate containing the CLI version of the search program.

**swcl_model**  
A library crate that will contain the model.

**swcl_gen**   
A binary crate, used to generate the model (Rust code) from text files.

</br>

# SWLC_GEN

## Introduction
This crate is part of the swl package and is used to generate a specific library (rust code). 
It automates the process of generating a Rust library based on user-supplied mixed-mode LTR/RTL text files. The advantage is also that any modification in the source files does not cause additional typing.

The generated library consists one function which makes is possible create a model of the SWLC for further processing.

## Usage

```
> swlc_gen     generates the library
> swlc_gen c   copies the library 
```
## HowTo

**Preconditions**

In order to generate a new version of the library yourself you will need the following:

1. Install the latest version of [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) on your system.
2. Download the latest version of the text format (`Tanach.txt.zip`) from [Unicode/XML Leningrad Codex (UXLC)](https://tanach.us/Pages/About.html).

</br>

**Steps to generate the library**

1. Download and unzip the `swlc` package from [GitHub](https://github.com/Roestdev/swlc)

2. Step into the swlc directory
``` rust
cd swlc
```

3. Unzip Tanach.txt.zip in the following directory: `swlc_gen/data/input`

4. Build the executable `swlc_gen`
  
``` rust
cargo build -p swlc_gen
```

5. Generate the library first
``` rust
cargo run -p swlc_gen
```

6. Copy the generated library into `swlc_core/src/`

``` rust
cargo run -p swlc_gen c
```


## License

The `swlc` package is distributed under either of

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT license](LICENSE-MIT)

at your option.

**Except** all biblical Hebrew text, this may be viewed or copied without restriction. 
For more information about their licence see [Tanach.us](https://tanach.us/License.html).

### Contribution<a name="contribution"></a>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.


## Questions, requests, bugs

I invite you to:

- Ask questions
- Make requests for new features or improvements
- Report bugs or suggest enhancements

Any input is welcome. To do this, you can submit an issue [here](https://github.com/Roestdev/swlc/issues).

