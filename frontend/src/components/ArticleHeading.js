import React from "react"
import Tooltip from "./Tooltip";

import { faCaretRight, faCaretDown } from "@fortawesome/free-solid-svg-icons"
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { autoUpdate, useFloating } from "@floating-ui/react-dom";
import { FloatingDelayGroup } from "@floating-ui/react";

export default function ArticleHeading(props) {
  const { item, isVisible } = props;
  const [isOpen, setIsOpen] = React.useState(false)
  const { refs, floatingStyles } = useFloating({
    whileElementsMounted: autoUpdate,
    open: isOpen,
    onOpenChange: setIsOpen
  });

  function toggleTooltip() {
    setIsOpen(prevIsOpen => !prevIsOpen)
  }

  return (
    <div className="article-collapsed-entry" key={item.doi}>
      <span className="caret">
        {isVisible ? <FontAwesomeIcon icon={faCaretDown} /> : <FontAwesomeIcon icon={faCaretRight} />}
      </span>
      <span className="title" ref={refs.setReference} onMouseEnter={toggleTooltip} onMouseLeave={toggleTooltip}>
        {item.title}
      </span>
      <span className="pub-date">{item.publication_date}</span>
      <span className="source">{item.source}</span>
    </div>
  )
}
