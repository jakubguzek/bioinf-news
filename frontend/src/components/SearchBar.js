import React from "react";

import AsyncCreatableSelect from "react-select/async-creatable";
import { components } from "react-select";
import { faMagnifyingGlass } from "@fortawesome/free-solid-svg-icons"
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

const DropdownIndicator = props => {
  return (
    <components.DropdownIndicator {...props}>
      <FontAwesomeIcon icon={faMagnifyingGlass} />
    </components.DropdownIndicator>
  );
};


export default function SearchBar(props) {
  const [ariaFocusMessage, setAriaFocusMessage] = React.useState("");

  function onFocus({ focused, isDisabled }) {
      const msg = `${isDisabled ? 'This option is diabled: ' : ''}
                    You are currently focused on option ${focused.label}`;
      setAriaFocusMessage(msg);
      return msg;
    };

  async function loadOptions(value) {
    const url = (value === null) ?
      "http://127.0.0.1:8080/articles" :
      `http://127.0.0.1:8080/articles?query=${value}`
    return fetch(url)
      .then(response => {
        if (response.ok) {
          return response.json()
        } else {
          return "{}".toJSON();
        }
      })
      .then(data => data.map(k => ({ value: k.title, label: k.title })));
  }

  function titleSelect(opt) {
    let title;
    if (!!opt) {
      title = opt.value;
    } else {
      title = null;
    }
    props.setTitle(title);
  }

  return (
    <div className="select-box">
      <AsyncCreatableSelect
        aria-label="Search for titles"
        ariaLiveMessages={{onFocus}}
        loadOptions={loadOptions}
        closeMenuOnSelect={false}
        inputId="search-title"
        name="Title Search Bar"
        isClearable
        cacheOptions
        components={{ DropdownIndicator }}
        allowCreateWhileLoading
        createOptionPosition="first"
        defaultOptions
        formatCreateLabel={inputValue => inputValue}
        onChange={opt => titleSelect(opt)}
        placeholder="Search for Title"
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
