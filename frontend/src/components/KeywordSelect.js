import React from "react";

import AsyncSelect from "react-select/async";

export default function KeywordSelect(props) {
  const [ariaFocusMessage, setAriaFocusMessage] = React.useState("");

  function onFocus({ focused, isDisabled }) {
      const msg = `${isDisabled ? 'This option is diabled: ' : ''}
                    You are currently focused on option ${focused.label}`;
      setAriaFocusMessage(msg);
      return msg;
    };

  async function loadOptions() {
    return fetch("http://127.0.0.1:8080/keywords")
      .then(response => response.json())
      .then(data => data.map(k => ({ value: k, label: k })));
  }

  function keywordSelect(opt) {
    const keywords = opt.map(o => o.value)
    props.setKeywords(keywords)
  }

  return (
    <div className="select-box">
      <AsyncSelect
        aria-label="Select keywords"
        ariaLiveMessages={{onFocus}}
        loadOptions={loadOptions}
        closeMenuOnSelect={false}
        isMulti
        onChange={opt => keywordSelect(opt)}
        placeholder="Select a Keyword"
        styles={{
          placeholder: (baseStyles, state) => ({
            ...baseStyles,
            display: 'flex',
          }),
        }}
      />
    </div>
  )
}
