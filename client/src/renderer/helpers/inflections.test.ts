/** @format */

import { kebabize } from "./inflections";

describe("kebabize", () => {
  it("should convert camelCase to kebab-case", () => {
    expect(kebabize("camelCase")).toEqual("camel-case");
  });

  it("should convert PascalCase to kebab-case", () => {
    expect(kebabize("PascalCase")).toEqual("pascal-case");
  });

  it("should handle abbreviations at start", () => {
    expect(kebabize("IDGenerator")).toEqual("id-generator");
  });

  it("should handle abbreviations at end", () => {
    expect(kebabize("uniqueID")).toEqual("unique-id");
  });

  it("should handle kebab-case", () => {
    expect(kebabize("kebab-case")).toEqual("kebab-case");
  });
});
