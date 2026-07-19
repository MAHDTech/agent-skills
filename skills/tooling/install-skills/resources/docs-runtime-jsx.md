\> \## Documentation Index \> Fetch the complete documentation index at:
https://bun.com/docs/llms.txt \> Use this file to discover all available
pages before exploring further. \# JSX \> Built-in JSX and TSX support
in Bun with configurable transpilation options Bun supports \`.jsx\` and
\`.tsx\` files. Bun's internal transpiler converts JSX syntax into
vanilla JavaScript before execution. \`\`\`ts react.tsx
icon="https://mintcdn.com/bun-1dd33a4e/JUhaF6Mf68z_zHyy/icons/typescript.svg?fit=max&auto=format&n=JUhaF6Mf68z_zHyy&q=85&s=7ac549adaea8d5487d8fbd58cc3ea35b"
theme={"theme":{"light":"github-light","dark":"dracula"}} function
Component(props: {message: string}) { return (

# {props.message}

); } console.log(); \`\`\` \## Configuration Bun reads your
\`tsconfig.json\` or \`jsconfig.json\` to determine how to perform the
JSX transform internally. If you'd rather not use either, you can set
the same options in \[\`bunfig.toml\`\](https://bun.com/runtime/bunfig). Bun respects
the following compiler options. \###
\[\`jsx\`\](https://www.typescriptlang.org/tsconfig#jsx) How JSX
constructs are transformed into vanilla JavaScript internally. The
following table lists the possible values of \`jsx\`, along with how
each transpiles this JSX component: \`\`\`tsx
theme={"theme":{"light":"github-light","dark":"dracula"}} Hello \`\`\`
\| Compiler options \| Transpiled output \| \|
--------------------------------------------------- \|
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
\| \| \`json  
{  
"jsx": "react"  
}  
\` \| \`tsx  
React.createElement(Box, { width: 5 }, "Hello");  
\` \| \| \`json  
{  
"jsx": "react-jsx"  
}  
\` \| \`tsx  
import { jsx } from "react/jsx-runtime";  
jsx("Box", { width: 5 }, "Hello");  
\` \| \| \`json  
{  
"jsx": "react-jsxdev"  
}  
\` \| \`tsx  
import { jsxDEV } from "react/jsx-dev-runtime";  
jsxDEV(  
"Box",  
{ width: 5, children: "Hello" },  
undefined,  
false,  
undefined,  
this,  
);  
\`  
  
The \`jsxDEV\` variable name is a React convention. The \`DEV\` suffix
marks code intended for development. The development version of React is
slower and includes additional validity checks & debugging tools. \| \|
\`json  
{  
"jsx": "preserve"  
}  
\` \| \`tsx  
// JSX is not transpiled  
// "preserve" is not supported by Bun currently  
Hello  
\` \| \###
\[\`jsxFactory\`\](https://www.typescriptlang.org/tsconfig#jsxFactory)

Only applicable when \`jsx\` is \`react\`.

The function name used to represent JSX constructs. Default value is
\`"React.createElement"\`. Set this for libraries like
\[Preact\](https://preactjs.com/) that use a different function name
(\`"h"\`). \| Compiler options \| Transpiled output \| \|
--------------------------------------------------------------------- \|
--------------------------------------------- \| \| \`json  
{  
"jsx": "react",  
"jsxFactory": "h"  
}  
\` \| \`tsx  
h(Box, { width: 5 }, "Hello");  
\` \| \###
\[\`jsxFragmentFactory\`\](https://www.typescriptlang.org/tsconfig#jsxFragmentFactory)

Only applicable when \`jsx\` is \`react\`.

The function name used to represent \[JSX
fragments\](https://react.dev/reference/react/Fragment) such as
\`\<\>Hello\</\>\`. Default value is \`"React.Fragment"\`. \| Compiler
options \| Transpiled output \| \|
-------------------------------------------------------------------------------------------------------------------
\|
------------------------------------------------------------------------------------------------
\| \| \`json  
{  
"jsx": "react",  
"jsxFactory": "myjsx",  
"jsxFragmentFactory": "MyFragment"  
}  
\` \| \`tsx  
// input  
\<\>Hello\</\>;  
  
// output  
myjsx(MyFragment, null, "Hello");  
\` \| \###
\[\`jsxImportSource\`\](https://www.typescriptlang.org/tsconfig#jsxImportSource)

Only applicable when \`jsx\` is \`react-jsx\` or \`react-jsxdev\`.

The module the component factory function (such as \`createElement\`,
\`jsx\`, or \`jsxDEV\`) is imported from. Default value is \`"react"\`.
You'll typically need this when using a component library like Preact.
\| Compiler options \| Transpiled output \| \|
----------------------------------------------------------------------------------------------------------------------
\|
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
\| \| \`jsonc  
{  
"jsx": "react-jsx",  
// jsxImportSource is not defined  
// default to "react"  
}  
\` \| \`tsx  
import { jsx } from "react/jsx-runtime";  
jsx("Box", { width: 5, children: "Hello" });  
\` \| \| \`jsonc  
{  
"jsx": "react-jsx",  
"jsxImportSource": "preact",  
}  
\` \| \`tsx  
import { jsx } from "preact/jsx-runtime";  
jsx("Box", { width: 5, children: "Hello" });  
\` \| \| \`jsonc  
{  
"jsx": "react-jsxdev",  
"jsxImportSource": "preact",  
}  
\` \| \`tsx  
// /jsx-runtime is automatically appended  
import { jsxDEV } from "preact/jsx-dev-runtime";  
jsxDEV(  
"Box",  
{ width: 5, children: "Hello" },  
undefined,  
false,  
undefined,  
this,  
);  
\` \| \### JSX pragma You can set any of these values per file with a
\*pragma\*, a comment that sets a compiler option in a particular file.
\| Pragma \| Equivalent config \| \|
---------------------------------------- \|
------------------------------------------------------------------ \| \|
\`ts  
// @jsx h  
\` \| \`jsonc  
{  
"jsxFactory": "h",  
}  
\` \| \| \`ts  
// @jsxFrag MyFragment  
\` \| \`jsonc  
{  
"jsxFragmentFactory": "MyFragment",  
}  
\` \| \| \`ts  
// @jsxImportSource preact  
\` \| \`jsonc  
{  
"jsxImportSource": "preact",  
}  
\` \| \## Logging Bun implements special logging for JSX to make
debugging easier. Given the following file: \`\`\`tsx index.tsx
icon="https://mintcdn.com/bun-1dd33a4e/JUhaF6Mf68z_zHyy/icons/typescript.svg?fit=max&auto=format&n=JUhaF6Mf68z_zHyy&q=85&s=7ac549adaea8d5487d8fbd58cc3ea35b"
theme={"theme":{"light":"github-light","dark":"dracula"}} import {
Stack, UserCard } from "./components"; console.log( , ); \`\`\` Bun
pretty-prints the component tree: \![JSX logging
output\](https://github.com/oven-sh/bun/assets/3084745/d29db51d-6837-44e2-b8be-84fc1b9e9d97)
\## Prop punning The Bun runtime also supports "prop punning" for JSX: a
shorthand for assigning a variable to a prop with the same name.
\`\`\`tsx react.tsx
icon="https://mintcdn.com/bun-1dd33a4e/JUhaF6Mf68z_zHyy/icons/typescript.svg?fit=max&auto=format&n=JUhaF6Mf68z_zHyy&q=85&s=7ac549adaea8d5487d8fbd58cc3ea35b"
theme={"theme":{"light":"github-light","dark":"dracula"}} function
Div(props: {className: string;}) { const {className} = props; // without
punning return

; // with punning return

; } \`\`\`
