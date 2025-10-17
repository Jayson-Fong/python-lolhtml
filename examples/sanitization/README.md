Sanitization
=================

This example leverages python-lolhtml to delete potentially dangerous elements and attributes.
We assume that only elements within a `main` tag could be dangerous. Further, we forbid tags besides `p` and `span`, 
which are common in blog posts. However, these tags could still contain malicious content, such as hidden text, 
prompting us to remove attributes.

Notice how the otherwise hidden message is revealed by removing the `style` tag whereas our known-good footer text's 
style is preserved. Additionally, the `script` tag is removed and its inner text is retained.

While this example loads all the content into the rewriter at once with one singular read, python-lolhtml also supports
streaming content and writing in chunks.

> [!CAUTION]
> It is generally a bad idea to make your own sanitizer, especially if your use case prompts a more permissive set of 
> rules. If you are not sure about what you are doing, consider using another purpose-built package like 
> [bleach](https://github.com/mozilla/bleach).