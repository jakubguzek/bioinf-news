import React from "react"

import { faCaretRight, faCaretDown } from "@fortawesome/free-solid-svg-icons"
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  useFloating,
  useHover,
  useInteractions,
  autoUpdate,
  offset,
  flip,
  shift,
  useFocus,
  useDismiss,
  useRole
} from "@floating-ui/react";

export default function ArticleHeading(props) {
  const { item, isVisible } = props;
  const [isShown, setIsShown] = React.useState(false);

  const delay = {open: 1000, close: 200};

  const { refs, floatingStyles, context } = useFloating({
    placement: "bottom",
    open: isShown,
    onOpenChange: setIsShown,
    middleware: [offset(10), flip(), shift()],
    whileElementsMounted: autoUpdate,
  });

  const hover = useHover(context, { move: false, delay });
  const focus = useFocus(context);
  const dismiss = useDismiss(context);
  const role = useRole(context, { role: "tooltip" });

  const { getReferenceProps, getFloatingProps } = useInteractions([
    hover,
    focus,
    dismiss,
    role,
  ]);

  return (
    <div className="article-collapsed-entry" key={item.doi}>
      <span className="caret">
        {isVisible ? <FontAwesomeIcon icon={faCaretDown} /> : <FontAwesomeIcon icon={faCaretRight} />}
      </span>
      <span className="title" ref={refs.setReference} {...getReferenceProps()}>
        {item.title}
      </span>
      {isShown && (<div className="title-tooltip" ref={refs.setFloating} style={floatingStyles} {...getFloatingProps()} >
        {item.title}
      </div>)}
      <span className="pub-date">{item.publication_date}</span>
      <span className="source">{item.source}</span>
    </div>
  )
}
