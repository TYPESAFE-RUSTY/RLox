<h1 align="center">RLox</h1>

<p align="center">
 This is the official repository for RLox.<br><br>
 <b>RLox is a programming language written in Rust, solely designed to make memes.</b>
</p>

<h2 align="center">Installation</h2>
Go To Actions section of this page :

![Go to Actions Section](./docs/installation_action.png)

Click on the latest Commit under workflow runs section :

![Click on the latest Commit](./docs/installation_commit_section.png)

Scroll to the bottom you will find Artifacts section click on rlox this will start downloading rlox.zip now you can use rlox!

![click on rlox](./docs/installation_rlox.png)


<h2 align="center">Usage</h2>
<h3 align="center">General</h3>
REPL:
Type 
<code>./rlox.exe</code> in the terminal to start repl mode which looks like :

![Repl Preview](./docs/usage_general_repl.png)

To run a file make sure rlox.exe is in same directory as your source file. You can run your file simply by typing <code>./rlox file_name</code> replace file_name with your file name example:

![File Structure](./docs/usage_general.png)


<h3 align="center">Dynamic Scanner</h3>

Define .tokenfile using rules defined [here](#dynamic_scanner)
example:

![File Structure](./docs/usage_dynamic_scanner.png)

this works in REPL mode too!:

![File Structure](./docs/usage_dynamic_scanner_repl.png)

yes this is similar to [bhialang](https://github.com/DulLabs/bhai-lang) you can find the preset for this in presets folder you can also add your presets to this repository for others to use refer [contribution guidelines](CONTRIBUTING.md) to raise pr.

<h2 align="center">Documentation</h2>

<h3 align="center">General</h3>
<p align="center">start of the file is the entrypoint for the program.</p>

```
// your code
// anything after '//' is comment
/* This is for multiline comment */ 
```

<h3 align="center">Variables</h3>
<p align="center">Variables can be declared using <code>var</code>.</p>

```
var a = 5 ;
var b = 6.9 ;
var c = "Hello World";
var d = true ;
var e = false ;
var f ;// nil is assigned by default

```

<h3 align="center">Types</h3>
<p align="center">Numbers strings and boolean are like other languages. Null values can be denoted using <code>nil</code>.

```
var a = 5 ; //integer
var b = 6.9 ; //float
var c = "Hello World"; //string
var d = true ; // boolean
var e = false ; // boolean
var f ; // nil is assigned by default

```

<h3 align="center">Built-ins</h3>
<p align="center">Use <code>print</code> to print anything to console.</p>

```
var a = 5;
print a ; //prints a to the terminal adds a \n by default so you don't have to manually
```

<h3 align="center">Conditionals</h3>
<p align="center">RLox supports if else blocks <code>if</code> block execute only if condition given is <code>true</code>. else block is optional and only executes if condition given is <code>false</code>.RLox also supports <code>and</code> and <code>or</code> logical operator.

```

var a = 5 ;

if (a == 5){
    print a ;
}
else {
    print b ;
}

```

<h3 align="center">Loops</h3>
<p align="center">Statements inside <code>while</code> blocks are executed as long as a specified condition evaluates to <code>true</code>. If the condition becomes <code>false</code>, statement within the loop stops executing and control passes to the statement following the loop.</p>


```

var a = 0;
while (a<10){
    a = a + 1;
    if (a==5){
        print a ;
    }
}
```

<h3 align="center" id="dynamic_scanner">Dynamic Scanner</h3>
<p align="center">When scanning the code RLox's scanner looks for a <code>.tokenfile</code> file in your directory and replaces reserved keyword with the tokens defined.Make sure tokens you replace are ones listed below.Pattern to replace tokens is as follows: Native_Token_Name : Your_Desired_Token_Name.Note token names should not contain any of these characters +,-,/,*,=,<,>,",{,},[,],(,) and whitespace characters.</p>

```
print : helloworld
```
<p>Now your program looks like:</p>

```
var a = "Print";
helloworld a ; //prints Print to the console and adds \n to the end.
```

<h3 align="center">Tokens</h3>

- [x] var
- [x] print
- [x] true
- [x] false
- [x] nil
- [x] if
- [x] else
- [x] or
- [x] and
- [x] while
- [ ] class
- [ ] for
- [ ] fun
- [ ] super
- [ ] this