use super::*;

use serde_json;
#[cfg(test)]
mod tests {
    use std::os::unix::raw::time_t;
    use super::*;

    #[test_log::test]
    fn test_parse_slides() {
        let input = r#"
        # Slide 1
        ```python
        @tanglit-block-def:hello_world
        print("Hello, world!")
        ```
        # Slide 2
        some content

        ---

        some *other* content
        # Slide 4
        the end

        --- ---

        final stuff

        ---

        foo bar

        # Slide 5
        Some other stuff

        Bla bla

        "#;

        let input_str = input.trim();
        let mdast = get_ast(input_str);
        for (i, child) in mdast.children().unwrap().iter().enumerate() {
            println!("i: {} {:?}", i, child);
        }
        let slides = get_slides(&mdast, input_str).unwrap();
        for (i, slide) in slides.iter().enumerate() {
            println!("Slide {}: {}", i, serde_json::to_string(slide).unwrap());
        }
    }

    #[test_log::test]
    fn test_exclude() {
        // env_logger::init();
        let input = r#"
# First and second slide title
foo
bla bla %
asdfasdf

wawawa
dflk *sj %
d* flkjsf
foofoo

---

some example
* a %i
* b
* c
* d

--- ---

asdkljsdfkj
sadlfkjasdklfj %
qweuoqwieu
# Fourth and fifth slide title
qwqwelw
asdsafd
qweqweqe

---

* asdfadf
* bebebe %i
* cececec

--- ---

* 11asdfadf
* 22bebebe
* 333cececec %i
asdfklsadñafljasdñfkj
 lakdsjfldkjshlsdjfhsdlfjkhsdaf
  asdklfsdlkjsdlkfjsdflksdfjlsdkjflsdkfj

---

  askdjlfhslkdjf
  asdkfljsdlkjfsdf
  sdfsdflkjqwerqweasdas
* sdflksjdflksjd %p
ladkjssldkfjsldkfj
sdlkfjsdlkfjsfl

---

  qweqoweiuqowieu
  weoriuworeiweuroiwe
lskdfjslkdfjslkdf
* adsfasfd
* adfasdfsdf
ñdfkgjañldkjasdf



        "#;
        let input_str = input.trim();
        process_slides(input_str);
    }

    #[test_log::test]
    fn test_another() {
        // env_logger::init();
        let input = r#"
# Slide 1 title (line 1)
slide1content (line 2)
slide1content (line 3)
slide1content (line 4) %

---
slide2content (line 7)
slide2content (line 8)
* listitem1 (line 9)
  listitem1 (line 10)
* listitem2 (line 11) %i
--- ---
slide3content (line 13)
slide3content (line 14)
        "#;
        let input_str = input.trim();
        let result = process_slides(input_str);
        assert!(result == vec![1, 6, 12], "Expected slide start lines to be [1, 6, 12], got {:?}", result);
    }
    #[test_log::test]
    fn test_another2() {
        // env_logger::init();
        let input = r#"
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg

# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg # first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa

lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlw
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg
# first slide
wawawawa
lkqjwhelqkwjhelqwkjehqlwkj hqwp eioupdoif asfñlqkeñr qjelkadjidsfq2e4 qjrladjfk gañdktjwñ 4lk5jqñewlkapd iudf pwñlke4rjtñ l4kñ1ql4k jqpèroi udfgsdfg

        "#;
        let input_str = input.trim();
        let start_millis = std::time::Instant::now();
        let result = process_slides(input_str);
        let end_millis = std::time::Instant::now();
        println!("Processing took {} ms", (end_millis - start_millis).as_millis());
    }

}
