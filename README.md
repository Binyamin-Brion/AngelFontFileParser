# AngelFontFileParser

Characters used for rendering are contained in a bitmap. The information to use a portion of the bitmap to render a character can be stored in an angel file.
This library is used to make that information usable using Rust constructs.

The type of block supported is type 4.

For more information, see https://www.angelcode.com/products/bmfont/doc/file_format.html.

Sample Use
---------------
```
let characters: Vec<CharacterInfo> = extract_characters(location_to_font_file).unwrap();

// The character information is stored using Option, as certain information
// might not able to be read successfully from the file

println!("{:?}", characters[0].id);
println!("{:?}", characters[0].width);
println!("{:?}", characters[0].height);

// print other character info...
```

Behavior for unexpected format
-----------------------------
* Additional paramter on character line: Extra paramter is ignored
* Character line missing parameter (such as width): CharacterInfo instance will not have a value for that parameter
* Mispelled parameter: CharacterInfo instance will not have a value for that parameter
* Character line does not start with 'char id': Line is ignored
* Invalid parameter format (is not of the value `parameterName=value`): CharacterInfo instance will not have a value for that parameter
