import { useCallback, useEffect, useRef, useState } from "react";
import ITool from "../../../models/ITool";
import IToolMenuItemCreation from "../../../models/IToolMenuItemCreation";
import styles from "./ToolMenuItemCreation.module.css";

/**
 * A component allowing creation of a new tool item in the tool menu
 */
const ToolMenuItemCreation: React.FC<IToolMenuItemCreation> = (props) => {
  const [newToolName, setNewToolName] = useState<string>("");
  const inputRef = useRef<HTMLInputElement | null>(null);

  useEffect(() => {
    if (props.isCreationActive) {
      focusInput();
    }
  }, [props.isCreationActive]);

  const keyDowns = useCallback((event: any) => {
    if (event.key === "Escape") {
      setNewToolName("");
      props.cancelCreation();
    }
  }, []);

  const focusInput = () => {
    if (inputRef.current) {
      inputRef.current?.focus();
      // For whatever reason, the browser needs little time
      // to select the input after focusing it
      setTimeout(() => {
        inputRef.current?.select();
      }, 10);
    }
  };
  useEffect(() => {
    document.addEventListener("keydown", keyDowns, false);

    return () => {
      document.removeEventListener("keydown", keyDowns, false);
    };
  }, [keyDowns]);

  const handleNewToolChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setNewToolName(event.target.value);
  };

  const confirmToolCreation = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    props.cancelCreation();
    if (!newToolName) {
      // log? shouldn't happen
      return;
    }
    let tool: ITool = { displayName: newToolName, tool: newToolName };
    props.newItemCreationHandler(tool);
    setNewToolName("");
  };

  return (
    <form
      className={
        props.isCreationActive
          ? styles["tool-menu__form--visible"]
          : styles["tool-menu__form--invisible"]
      }
    >
      <div className={styles["tool-menu__new-tool"]}>
        <input
          name="new_tool_input"
          className={styles["tool-menu__new-tool-input"]}
          type="text"
          onChange={handleNewToolChange}
          value={newToolName}
          ref={inputRef}
          onBlur={() => {
            props.cancelCreation();
            setNewToolName("");
          }}
        />
      </div>
      <button
        className={styles["tool-menu__input-confirmation-button"]}
        onClick={confirmToolCreation}
      ></button>
    </form>
  );
};

export default ToolMenuItemCreation;
