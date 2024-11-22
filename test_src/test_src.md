# test_src
- these are just test files to point the comment-away bin at to see if it does it's job correctly.
- i recommend copying these to a new directory like so:
    - ```cp -r test_src/ test/```
- run parser-scraper:
    - ```
      mkdir shared_libs_src
      mkdir shared_libs
      ./parser-scraper -l python,rust,go,javascript,java
      ``` 
- then run comment-away:
    - ```./comment-away -t test```
