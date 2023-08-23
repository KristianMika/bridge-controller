import styles from "./InterfaceConfiguration.module.css";
import Switch from "react-switch";
import "react-dropdown/style.css";
import Creatable from "react-select/creatable";
import React, { useEffect, useState } from "react";
import {
  CryptographicInterface,
  getInterfaceConfiguration,
  Group,
  getGroups,
  setInterfaceConfiguration,
} from "../../bindings";
import Select from "react-select";
import { MultilineSelectOption } from "./MultilineSelectOption/MultilineSelectOption";
import { CertificateUpload } from "./CertificateUpload";

const HEX_PUBKEY_DISPLAY_CHARS_COUNT = 10;
interface IFormData {
  isEnabled: boolean;
  communicatorUrl: string;
  selectedGroup: string;
}
const DEFAULT_COMMUNICATOR_URLS = ["meesign.crocs.fi.muni.cz", "localhost"];

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
  communicatorUrl: "",
  selectedGroup: "",
};

export const InterfaceConfiguration: React.FC<IInterfaceConfiguration> = (
  props
) => {
  const [formData, setFormData] = useState<IFormData>(() => {
    return { ...defaultFormData };
  });
  const [groups, setGroups] = useState<Group[]>([]);
  const [options, setOptions] = useState(
    createOptions(DEFAULT_COMMUNICATOR_URLS)
  );

  const handleIsEnabledChange = (checked: boolean) => {
    setFormData((prev: IFormData) => {
      return { ...prev, isEnabled: checked };
    });
  };

  const setCommunicatorUrl = (url: string) => {
    setFormData((prev) => {
      return { ...prev, communicatorUrl: url };
    });
  };

  const handleGroupChange = (event: OptionType) => {
    setFormData((prev: IFormData) => {
      return { ...prev, selectedGroup: event.value };
    });
  };

  useEffect(() => {
    getInterfaceConfiguration(props.interfaceType).then((configuration) => {
      if (!configuration) {
        return;
      }
      setFormData(configuration);
      if (configuration!.communicatorUrl) {
        getGroups(configuration!.communicatorUrl).then((groups) => {
          setGroups(groups);
        });
      }
    });
  }, []);

  const handleCommunicatorUrlCreation = (inputValue: string) => {
    setCommunicatorUrl(inputValue);
    setOptions((prev) => [...prev, createOption(inputValue)]);
  };

  const saveConfiguration = (event: React.MouseEvent<HTMLElement>) => {
    event.preventDefault();
    setInterfaceConfiguration(props.interfaceType, formData);
  };

  const handleCommunicatorUrlChange = (newValue: any) => {
    setGroups([]);
    setFormData((prev) => {
      return { ...prev, selectedGroup: "" };
    });
    getGroups(newValue.value).then((groups) => {
      setGroups(groups);
    });
    setCommunicatorUrl(newValue.value);
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
          className={styles["form__communicator_input"]}
          value={createOption(formData.communicatorUrl)}
          onChange={handleCommunicatorUrlChange}
          onCreateOption={handleCommunicatorUrlCreation}
          name="communicatorUrl"
          options={options as any}
        ></Creatable>

        <label className={styles["form__communicator_input_label"]}>
          Communicator URL
        </label>
        <CertificateUpload
          className={styles["form__communicator_file_upload_button"]}
          isDisabled={!formData.isEnabled || !formData.communicatorUrl}
          communicatorUrl={formData.communicatorUrl}
        />

        <label
          className={styles["form__communicator_file_upload_button_label"]}
        >
          Communicator Cert
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
          isDisabled={!formData.isEnabled || !formData.communicatorUrl}
          onChange={handleGroupChange}
          components={{ Option: MultilineSelectOption }}
          value={createOption(formData["selectedGroup"])} // TODO: display name, not pubkey
        />
        <label className={styles["form__select_pubkey_label"]}>Group</label>
        <button onClick={saveConfiguration} className={styles["form__apply"]}>
          Save
        </button>
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
