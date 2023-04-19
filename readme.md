
<h3>OOP-style inheritance in Rust</h3>
Unlike object-oriented languages, Rust does not support the concept of inheritance.
If you are used to using OOP design patterns, then you have to find another way in Rust.<br/>
However, in cases where you only need a two-level inheritance hierarchy, you can actually achieve something similar in Rust.
A two-level hierarchy would be a base class and some extensions of that base class, or a base interface with some implementations of it.
In Rust I will refer to the base class/interface as the "base type".

Some good reasons (also in Rust) to create a two-level "hierarchy" of entities are:
* You can use the base type for function parameters.
* You can insert structs of different sub-types into the same collection (such as a vector).
* You can call a common function on the base type. 

Rust has two methods for this. You can either use enum or traits as a replacement for the base class.
The enum variant is usually the easiest one to work with, so I would recommend this in most cases.
However, if you are making a library where the user of the library should be able to create his own implementations of your base type, then traits is the way to go.

Note that the enum variant often has better performance when used for collection items, as the items in the collection will be next to each other in memory, thereby improving cache efficiency.
To check what sub-type an entity has is also easier with enums, as you simply compare the enum.
With traits you have to check like: <code>shape.type_id() == TypeId::of::&lt;Rectangle>()</code> <br/>
On the negative side, the enum variant may use more memory, as all sub-structs are padded to the same size.

The provided sample files show two implementations of a typical OOP example with different geometric shapes. In OOP they would all inherit from a shape base class.<br/>
* The file <code>shape_enum.rs</code> shows the enum approach.
* The file <code>shape_trait.rs</code> shows the trait approach.
* The file <code>main.rs</code> shows how each variant can be used.