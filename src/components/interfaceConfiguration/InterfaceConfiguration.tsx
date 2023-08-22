import styles from "./InterfaceConfiguration.module.css";
import Switch from "react-switch";
import "react-dropdown/style.css";
import Creatable from "react-select/creatable";
import { open } from "@tauri-apps/api/dialog";
import React, { useEffect, useState } from "react";
import {
  CryptographicInterface,
  getInterfaceConfiguration,
  Group,
  getGroups,
  setCommunicatorCertificatePath,
} from "../../bindings";
import Select, { OnChangeValue } from "react-select";
import { MultilineSelectOption } from "./MultilineSelectOption/MultilineSelectOption";

const HEX_PUBKEY_DISPLAY_CHARS_COUNT = 10;
interface IFormData {
  isEnabled: boolean;
  controllerUrl: string;
  selectedPublicKey: string;
}
const MEESIGN_URLS = ["meesign.crocs.fi.muni.cz", "localhost"];

interface IInterfaceConfiguration {
  canBeDisabled: boolean;
  interfaceType: CryptographicInterface;
}
interface Option {
  readonly label: string;
  readonly value: string;
}

const createOption = (option: string): Option => {
  return { value: option, label: option };
};

const createOptions = (options: string[]): Option[] => {
  return options.map(createOption);
};

const defaultFormData: IFormData = {
  isEnabled: true,
  controllerUrl: "",
  selectedPublicKey: "",
};

export const InterfaceConfiguration: React.FC<IInterfaceConfiguration> = (
  props
) => {
  const [formData, setFormData] = useState<IFormData>(() => {
    return { ...defaultFormData };
  });
  const [groups, setGroups] = useState<Group[]>([]);
  const [options, setOptions] = useState(createOptions(MEESIGN_URLS));

  const handleIsEnabledChange = (checked: boolean) => {
    setFormData((prev: IFormData) => {
      return { ...prev, isEnabled: checked };
    });
  };

  const setControllerUrl = (url: string) => {
    setFormData((prev) => {
      return { ...prev, controllerUrl: url };
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
      if (configuration!.controller_url) {
        getGroups(configuration!.controller_url).then((groups) => {
          setGroups(groups);
        });
      }
    });
  }, []);

  const uploadFile = (event: React.MouseEvent<HTMLElement>) => {
    event.preventDefault();
    open({
      multiple: false,
      directory: false,
      filters: [{ name: "PEM Certificates", extensions: ["pem"] }],
    }).then((filePath) => {
      if (filePath && typeof filePath === "string") {
        setCommunicatorCertificatePath(filePath);
      }
    });
  };

  const handleOptionCreate = (inputValue: string) => {
    setControllerUrl(inputValue);
    setOptions((prev) => [...prev, createOption(inputValue)]);
  };
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
        <Creatable
          isDisabled={!formData.isEnabled}
          className={styles["form__controler_input"]}
          value={formData.controllerUrl}
          onChange={(newValue) => {
            setControllerUrl(newValue as string);
          }}
          onCreateOption={handleOptionCreate}
          name="controllerUrl"
          options={options as any}
        ></Creatable>
        <label className={styles["form__controler_input_label"]}>
          Controller URL
        </label>
        <button
          onClick={uploadFile}
          className={styles["form__controler_file_upload_button"]}
        >
          Upload
        </button>
        <label className={styles["form__controler_file_upload_button_label"]}>
          Controller Cert
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
          isDisabled={!formData.isEnabled || !formData.controllerUrl}
          onChange={handleDropDownChange}
          components={{ Option: MultilineSelectOption }}
          value={formData["selectedPublicKey"]}
        />
        <label className={styles["form__select_pubkey_label"]}>Group</label>
        <button className={styles["form__apply"]}>Save</button>
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
