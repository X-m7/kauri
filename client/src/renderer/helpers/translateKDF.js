/**
 * Translate KDF nodes into Redux ready objects
 *
 * Recursively traverses tree of KDF nodes, assigning them unique IDs and adding
 * them to byId and allIds.
 *
 * // {
 * //   byId: {
 * //     1: { id: 1, type: "text", content: "Hello, world!" }
 * //   },
 * //   allIds: [1]
 * // }
 *
 * @format
 * @example translateKDF([ { type: "text", content: "Hello, world!" } ]);
 * @param {Object[]} nodes An array of KDF nodes.
 * @return A byId map, and an allIds list.
 */

export function translateKDF(nodes) {
  let id = 0;
  const byId = {};

  // Recursive callbacks
  const nextId = () => id++;
  const addToById = node => (byId[node.id] = node);

  // Recursively translate nodes
  const allIds = nodes.map(node => translateKDFNode(node, nextId, addToById));
  return { byId, allIds };
}

/**
 * Translate a KDF node into a Redux ready object.
 *
 * Flattens a node and assign it an ID, then recursively travels to and
 * translates any of the nodes children.
 *
 * @param {Object} node KDF node to translate.
 * @param {function(): number} nextId A callback to generate a new id.
 * @param {function(node: Object)} addToById Adds nodes to byID map
 * @return Node's assigned ID.
 */
export function translateKDFNode(node, nextId, addToById) {
  // Handle text shorthand
  if (typeof node === "string") {
    node = {
      type: "text",
      content: node,
    };
  }

  // Assign id and translate children
  node.id = nextId();
  if (node.children) {
    node.children = node.children.map(node =>
      translateKDFNode(node, nextId, addToById),
    );
  }

  addToById(node);
  return node.id;
}
