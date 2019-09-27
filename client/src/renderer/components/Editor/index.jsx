/** @format */

import "./styles.scss";

import { h, Component, createRef } from "preact";
import { connect } from "react-redux";
import { moveSelection, Status } from "redux/actions";
import { Renderer, RenderMode } from "render";

import Error from "components/Error";

/**
 * A document editing component.
 * @extends Component
 */
class Editor extends Component {
  /**
   * Constructs a new editor component.
   */
  constructor(props) {
    super(props);
    this.contentEditableDiv = createRef();

    // Binds
    this.handleDocumentClick = this.handleDocumentClick.bind(this);
    this.logKeyPress = this.logKeyPress.bind(this);
  }

  /**
   * Returns absolute values of caret's start/end positions
   */
  getCaretPos() {
    const range = document.getSelection().getRangeAt(0);
    const preSelectionRange = range.cloneRange();
    preSelectionRange.selectNodeContents(this.contentEditableDiv.current);
    preSelectionRange.setEnd(range.startContainer, range.startOffset);
    const positionStart = preSelectionRange.toString().length;
    const positionEnd = positionStart + range.toString().length;
    return [positionStart, positionEnd];
  }

  /**
   * Used to queue actions that need fire after React's call stack has completely resolved
   * @param {function()} callback
   */
  onNextFrame(callback) {
    setTimeout(function() {
      requestAnimationFrame(callback);
    });
  }

  /**
   * Handles clicks to the document element.
   */
  handleDocumentClick() {
    this.onNextFrame(() => {
      this.props.moveSelection(...this.getCaretPos());
    });
  }

  /**
   * Listens to keyboard presses
   * @param {number} e
   */
  logKeyPress(e) {
    console.log(e);
    switch (e.keyCode) {
      //arrow keys
      case 37:
      case 39:
      case 38:
      case 40:
        this.onNextFrame(() => {
          this.props.moveSelection(...this.getCaretPos());
        });
        break;
    }
  }

  render(props) {
    let content;

    switch (props.document.status) {
      case Status.ERROR:
        content = <Error exception={props.document.exception} />;
        break;
      default:
        const pages = new Renderer(props.document.content, {
          renderMode: RenderMode.CONTENT,
          pageStyle: {
            marginBottom: "1cm",
            marginLeft: "1cm",
            marginRight: "1cm",
            marginTop: "1cm",
            height: "140mm",
            width: "120mm",
          },
        }).render();

        content = (
          <div
            ref={this.contentEditableDiv}
            class="editor"
            contenteditable="true"
            onClick={this.handleDocumentClick}
            onkeyDown={this.logKeyPress}
          >
            {pages}
          </div>
        );
        break;
    }

    return content;
  }
}

export default connect(
  state => ({ document: state.document }),
  { moveSelection },
)(Editor);
