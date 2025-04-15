Allows the retrieval of trait implementations.

Still very much a work in progress.

## Explanation of this madness:
Imagine if you could store a list of types, consts, and functions, all at compile time.
There are various ways of accomplishing that, but now what if instead you could automatically generate that list from desired items located anywhere in your crate?
It is possible, using this crate.

How? Simple, we create a trait that holds the items we want to collect. Then we use an attribute proc macro, that you put on each trait implementation containing the items you want to send to the list.
Every invocation of the attribute proc macro assumes that it is the last. When it gets invoked again, it simply unimplements the last invocation.

Here is roughly an example of the techniques we use for this: TO DO: Insert playground link and rust code block.
This unfortunately does require us to be able to count in the proc macro, which sadly means we have to use static abuse...
This may not work with proc macro caching, and in fact could stop working at any moment. We are hopeful that rust will add proper state to proc macros before they break this trick.
