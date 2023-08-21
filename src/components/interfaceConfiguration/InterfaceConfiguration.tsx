import styles from "./InterfaceConfiguration.module.css";
import Switch from "react-switch";

import "react-dropdown/style.css";

import React, { useEffect, useState } from "react";
import {
  CryptographicInterface,
  getInterfaceConfiguration,
  Group,
  getGroups,
} from "../../bindings";
import Select from "react-select";
import { MultilineSelectOption } from "./MultilineSelectOption/MultilineSelectOption";

const HEX_PUBKEY_DISPLAY_CHARS_COUNT = 10;
interface IFormData {
  isEnabled: boolean;
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
    controllerUrl: "",
    selectedPublicKey: "default key",
  };

  const [formData, setFormData] = useState<IFormData>(() => {
    return { ...defaultFormData };
  });

  const [groups, setGroups] = useState<Group[]>([]);
  const handleIsEnabledChange = (checked: boolean) => {
    setFormData((prev: IFormData) => {
      return { ...prev, isEnabled: checked };
    });
  };

  const handleChange = (event: React.FormEvent) => {
    const name = (event.target as HTMLTextAreaElement).name;
    const value = (event.target as HTMLTextAreaElement).value;
    setFormData((prev) => {
      return { ...prev, [name]: value };
    });
  };

  const handleDropDownChange = (event: OptionType) => {
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

    // getGroups(formData.controllerUrl).then((groups) => {
    getGroups("meesign.local").then((groups) => {
      setGroups(groups);
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

        <Select
          options={groups.map((group) => {
            return {
              label: group.name,
              subLabel: shortenHexPubkey(group.group_id),
              value: group.group_id,
            };
          })}
          placeholder="Select an option"
          className={styles["form__select_pubkey"]}
          isDisabled={!formData.isEnabled}
          onChange={handleDropDownChange}
          components={{ Option: MultilineSelectOption }}
        />

        <label className={styles["form__select_pubkey_label"]}>
          Public key
        </label>
        <button className={styles["form__apply"]}>Apply</button>
      </form>
    </div>
  );
};

interface OptionType {
  label: string;
  subLabel: string;
  value: string;
}

const shortenHexPubkey = (pubkey: string): string => {
  return (
    pubkey.slice(0, HEX_PUBKEY_DISPLAY_CHARS_COUNT) +
    "..." +
    pubkey.slice(-HEX_PUBKEY_DISPLAY_CHARS_COUNT)
  );
};
