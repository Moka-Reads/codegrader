# CodeGrader

> This project's licensing terms are subject to MoKa Reads, please visit [MoKa Reads Licenses](https://mokareads.org/license) for more information

The goal of this project is to create a code grader that will utilize Rust's unit testing to allow a student 
to edit a file and use `cargo test --test <question>` to check if their implementation is correct. 

The structure is the following: 
```shell
codegrader/ 
  src/ 
    lib.rs 
    q<num>.rs 
    ...
  tests/ 
    q<num>.rs 
    ...
```

Considerations: 
- Create a way for an instructor to define test parameters and expected results using a `json` or `toml` file 
and have the tests use them. 

- Create an interactive system for a student to use such as asking them what question they'd like to try to check,
give them a description of what needs to be done and hints. 
- If these are to be a way for students to learn and not be graded, we could also have documentation to the solution 
and reasons of the implementation. 

