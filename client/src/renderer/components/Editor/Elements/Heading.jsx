/** @format */

import { h } from "preact";
import { Element } from "./index";

export function Heading(props) {
  // Ensure level is valid
  if (props.level < 1) {
    throw `Invalid heading level '${props.level}'`;
  }

  return <Element tag={`h${Math.min(props.level, 6)}`} {...props} />;
}
