# Lorem-Ipsum-rs Generator

Quickly generate placeholder text using Rust

## Installation

`cargo install lorem-ipsum`

### Upgrading

`cargo install --force lorem-ipsum`

## Usage

```txt
Usage: lorem-ipsum [OPTIONS]

Options:
  -t, --text-mode <TEXT_MODE>
          What kind of text to generate
          
          [default: paragraphs]

          Possible values:
          - words:      Generate a number of words specified by word-count
          - sentences:  Generate a number of sentences specified by sentence-count
          - paragraphs: Generate a number of paragraphs specified by paragraph-count

  -W, --wordlist <WORDLIST>
          A path to a wordlist to use other than the default

  -w, --words <WORDS_COUNT>
          Specify how many words in total / per sentence to generate
          
          [default: 7..15]

  -s, --sentences <SENTENCES_COUNT>
          Specify how many sentences into total / per paragraph to generate
          
          [default: 3..5]

  -p, --paragraphs <PARAGRAPHS_COUNT>
          Specify how many paragraphs to generate in total
          
          [default: 2]

  -c, --comma <COMMA_PROBABILITY>
          Specify about how many words to generate before placing a comma
          
          [default: 7]

  -f, --static-first-sentence
          Whether or not the first sentence / words should be the standard "Lorem ipsum..."

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Example usage

```txt
$ lorem-ipsum
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Cupidatat eu consectetur lorem irure reprehenderit nisi id velit et consequat. Aute dolor minim cupidatat tempor nulla sit in. Quis tempor, sit, eu aliquip ut fugiat est ullamco laboris irure exercitation aute.

Voluptate ullamco veniam elit ea aliquip sit. Ipsum eu dolor dolore, amet laborum qui nostrud, quis labore. Reprehenderit sit ex sint commodo dolore, pariatur qui est fugiat veniam velit laborum ea labore.
```

Or, to get a wrapped, we can use the GNU `fold` command:

```txt
$ lorem-ipsum | fold -w 80 -s
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor 
incididunt ut labore et dolore magna aliqua. Veniam culpa officia ipsum qui 
nisi labore ex eiusmod, dolor consequat. Amet consectetur tempor cillum duis 
incididunt mollit ex.

Ut sit labore do velit eu, lorem id in enim dolore ea eiusmod cupidatat irure. 
Deserunt eiusmod velit et qui veniam nostrud duis ea elit dolore ullamco 
adipiscing. Elit voluptate velit enim laborum, culpa duis.
```
