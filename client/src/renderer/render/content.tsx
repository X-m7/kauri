/** @format */

import { h, render } from "preact";
import { RenderMode } from "./index";
import { convertToPixels } from "helpers/units";
import store from "redux/store";
import { cacheNode, cacheWorkingArea } from "redux/actions/cache";

import * as Nodes from "components/Editor/Nodes";
import * as Elements from "components/Editor/Elements";
import Page from "components/Editor/Page";

/**
 * An interface for describing a page's styles.
 */
export interface PageStyle {
  height: string;
  marginBottom: string;
  marginLeft: string;
  marginRight: string;
  marginTop: string;
  width: string;
}

/**
 * An interface which describes possible renderer options.
 */
export interface RendererOptions {
  renderMode: RenderMode;
  pageStyle: PageStyle;
}

export interface Node {
  type: string;
}

export interface DocumentState {
  byId: { [id: number]: Node };
  allIds: number[];
}

type Renderable = { type: any; props: {} } | string;

export interface Component {
  [attribute: string]: any;
}

/**
 * A map of node types to components.
 *
 * It's faster to perform a lookup in an object when you know the key, than
 * create a giant switch case statement with each possible Node type. Having a
 * lookup object also allows us to create additional nodes at runtime. This
 * could prove valuable once we begin supporting third-party extensions.
 */
const NODE_MAP: { [type: string]: any } = {
  caption: Elements.Caption,
  code: Elements.InlineCode,
  codeblock: Elements.CodeBlock,
  heading: Elements.Heading,
  hint: Elements.Hint,
  hyperlink: Elements.Hyperlink,
  linebreak: Nodes.LineBreak,
  list: Elements.List,
  listitem: Elements.ListItem,
  pagebreak: Nodes.PageBreak,
  paragraph: Elements.Paragraph,
  span: Elements.Span,
  table: Elements.Table,
  tablecell: Elements.TableCell,
  tablerow: Elements.TableRow,
  text: Nodes.Text,
};

/**
 * A renderer for KDF nodes.
 *
 * @description
 * The renderer has a number of major responsibilities, which it must complete
 * within a timely manner to prevent the user experiencing any input lag. These
 * responsibilities can be loosly defined as the following:
 *
 *  1. Determine when to wrap content to another page.
 *  2. Use a scratch render to predict the size of a node.
 *  3. Use caching to skip the above step whenever possible.
 *  4. Know when to invalidate a node's cache.
 *  5. Handle the possible render modes.
 *
 * @example
 * new Renderer(options).render(nodes);
 */
export class Renderer {
  private workingHeight: number;
  private workingWidth: number;

  /**
   * Constructs a new Renderer.
   * @param nodes Document nodes.
   * @param options Optional renderer configuration.
   *
   * @example
   * new Renderer({ pageStyle, renderMode: RenderMode.CONTENT })
   */
  constructor(private nodes: DocumentState, private options: RendererOptions) {
    const cache = store.getState().cache.workingArea;

    if (cache.didInvalidate) {
      // Calculate available working height and width
      this.workingHeight =
        convertToPixels(options.pageStyle.height) -
        convertToPixels(options.pageStyle.marginTop) -
        convertToPixels(options.pageStyle.marginBottom);
      this.workingWidth =
        convertToPixels(options.pageStyle.width) -
        convertToPixels(options.pageStyle.marginLeft) -
        convertToPixels(options.pageStyle.marginRight);
      store.dispatch(cacheWorkingArea(this.workingWidth, this.workingHeight));
    } else {
      this.workingHeight = cache.height;
      this.workingWidth = cache.width;
    }

    // Binds
    this.render = this.render.bind(this);
    this.toRenderable = this.toRenderable.bind(this);
    this.renderNodeChildren = this.renderNodeChildren.bind(this);
    this.scratchRender = this.scratchRender.bind(this);
  }

  /**
   * Convert a KDF node to preact renderable.
   * @param node KDF node to convert to a preact renderable.
   */
  private toRenderable(node: Node): Renderable {
    // Handle text node shorthand
    if (typeof node === "string") {
      return node;
    }

    // Ensure type is lowercase
    const type = node.type.toLowerCase();

    // Handle unknown type
    if (!(type in NODE_MAP)) {
      throw `Unknown element type '${node.type}'.`;
    }

    // Create and return preact component
    const NodeTag = NODE_MAP[type];
    return <NodeTag {...node} renderChildren={this.renderNodeChildren} />;
  }

  /**
   * Renders a KDF node's children.
   * @param children An array of KDF node ids.
   */
  private renderNodeChildren(children: number[]): Renderable[] {
    return children.map(id => this.toRenderable(this.nodes.byId[id]));
  }

  /**
   * Render a component in the scratch area to determine it's size.
   * @param component Component to render.
   */
  private scratchRender(component: Component): [number, number, number] {
    let scratchArea: HTMLElement = document.querySelector(".__scratch");

    if (!scratchArea) {
      scratchArea = document.createElement("div");
      scratchArea.classList.add("__scratch");
      scratchArea.style.visibility = "hidden";
      scratchArea.style.width = `${this.workingWidth}px`;
      document.body.appendChild(scratchArea);
    }

    render(component, scratchArea);
    const computedStyles = window.getComputedStyle(component.__e);
    return [
      component.__e.getBoundingClientRect().height,
      parseInt(computedStyles.marginTop),
      parseInt(computedStyles.marginBottom),
    ];
  }

  /**
   * Render KDF nodes to paginated preact element list.
   */
  render() {
    const renderables: Renderable[] = [];
    const pages: Renderable[][] = [];
    const cache = store.getState().cache;
    const { nodes } = this;

    let currentPage: Renderable[] = [];
    let currentHeight = this.workingHeight;

    // Convert each root node into renderables
    nodes.allIds.forEach(id => {
      renderables.push(this.toRenderable(nodes.byId[id]));
    });

    // Scratch render any nodes that aren't cached
    renderables.forEach(renderable => {
      // We only want to deal with Preact Components now.
      if (typeof renderable === "string") {
        return;
      }

      // Handle page break
      if (renderable.type === Nodes.PageBreak) {
        pages.push(currentPage);
        currentPage = [];
        currentHeight = this.workingHeight;
        return;
      }

      let height, marginTop, marginBottom;
      const component = renderable as Component;
      const { id } = component.props;

      // Check if cache contains renderable
      if (!(id in cache.nodesById) || cache.nodesById[id].didInvalidate) {
        [height, marginTop, marginBottom] = this.scratchRender(component);
        store.dispatch(
          cacheNode(component.props.id, { height, marginTop, marginBottom }),
        );
      } else {
        ({ height, marginTop, marginBottom } = cache.nodesById[id]);
      }

      // Only subtract marginTop if not the first child
      if (currentHeight !== this.workingHeight) {
        currentHeight -= marginTop;
      }
      currentHeight -= height;

      // Push new page if adding this renderable would cause overflow
      if (currentHeight < 0) {
        pages.push(currentPage);
        currentPage = [renderable];
        currentHeight = this.workingHeight - height - marginBottom;
        return;
      }

      currentHeight -= marginBottom;
      currentPage.push(renderable);
    });

    // Render content to pages
    return pages.map(page => (
      <Page children={page} styles={this.options.pageStyle} />
    ));
  }
}
