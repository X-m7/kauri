/** @format */

import { h } from "preact";
import { Element } from "./index";

export const InlineCode = props => <Element tag="code" {...props} />;

// TODO Syntax highlighting
// TODO Line numbers
// TODO File name
export const CodeBlock = props => (
  <pre>
    <Element tag="code" {...props} />
  </pre>
);
