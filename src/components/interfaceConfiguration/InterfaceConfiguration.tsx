import styles from "./InterfaceConfiguration.module.css";
import Switch from "react-switch";
import Dropdown, { Option } from "react-dropdown";
import "react-dropdown/style.css";
import { useEffect, useState } from "react";
import {
  setInterfaceConfiguration,
  InterfaceConfiguration as InterfaceConfigurationType,
  CryptographicInterface,
  getInterfaceConfiguration,
} from "../../bindings";

interface IFormData {
  isEnabled: boolean;
  isPassThroughtEnabled: boolean;
  controllerUrl: string;
  selectedPublicKey: string;
}

interface IInterfaceConfiguration {
  canBeDisabled: boolean;
  interfaceType: CryptographicInterface;
}

export const InterfaceConfiguration: React.FC<IInterfaceConfiguration> = (
  props
) => {
  const defaultFormData: IFormData = {
    isEnabled: true,
    isPassThroughtEnabled: true,
    controllerUrl: "",
    selectedPublicKey: "default key",
  };

  const [formData, setFormData] = useState<IFormData>(() => {
    return { ...defaultFormData };
  });
  const handleIsEnabledChange = (checked: boolean) => {
    setFormData((prev: IFormData) => {
      return { ...prev, isEnabled: checked };
    });
  };

  const handleIsPassThroughEnabledChange = (checked: boolean) => {
    setFormData((prev: IFormData) => {
      return { ...prev, isPassThroughtEnabled: checked };
    });
  };

  const handleChange = (event: React.FormEvent) => {
    const name = (event.target as HTMLTextAreaElement).name;
    const value = (event.target as HTMLTextAreaElement).value;
    setFormData((prev) => {
      return { ...prev, [name]: value };
    });
  };

  const handleDropDownChange = (event: Option) => {
    setFormData((prev: IFormData) => {
      return { ...prev, selectedPublicKey: event.value };
    });
  };

  useEffect(() => {
    getInterfaceConfiguration(props.interfaceType).then((configuration) => {
      if (!configuration) {
        return;
      }
      setFormData((prev: IFormData) => {
        return { ...prev, controllerUrl: configuration!.controller_url };
      });
    });
  }, []);

  return (
    <div className={styles["interface-configuration"]}>
      <form className={styles["interface-configuration__form"]}>
        <Switch
          className={styles["form__enabled"]}
          onChange={handleIsEnabledChange}
          checked={formData.isEnabled}
          disabled={!props.canBeDisabled}
        />
        <label className={styles["form__enabled_label"]}>Enabled</label>
        <input
          disabled={!formData.isEnabled}
          className={styles["form__controler_input"]}
          placeholder="Controller URL"
          type="text"
          value={formData.controllerUrl}
          onChange={handleChange}
          name="controllerUrl"
        ></input>
        <label className={styles["form__controler_input_label"]}>
          Controller URL
        </label>

        <label className={styles["form__autopass_label"]}>
          Auto pass throught
        </label>
        <Switch
          className={styles["form__autopass"]}
          onChange={handleIsPassThroughEnabledChange}
          checked={formData.isPassThroughtEnabled}
          disabled={!formData.isEnabled}
        />

        <Dropdown
          options={["default key", "secondary key"]}
          placeholder="Select an option"
          className={styles["form__select_pubkey"]}
          disabled={!formData.isEnabled}
          onChange={handleDropDownChange}
        />

        <label className={styles["form__select_pubkey_label"]}>
          Public key
        </label>
        <button className={styles["form__apply"]}>Apply</button>
      </form>
    </div>
  );
};
