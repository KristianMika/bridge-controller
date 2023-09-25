import styles from "./InterfaceConfiguration.module.css";
import Switch from "react-switch";
import "react-dropdown/style.css";
import Creatable from "react-select/creatable";
import { Theme } from "react-select";
import React, { useEffect, useState, CSSProperties } from "react";
import {
  CryptographicInterface,
  getInterfaceConfiguration,
  Group,
  getGroups,
  setInterfaceConfiguration,
  spawnInterfaceProcess,
  killInterfaceProcess,
  CreatableInterface,
} from "../../bindings";
import Select, { StylesConfig } from "react-select";
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
  displayName: string;
}
interface Option {
  readonly label: string;
  readonly value: string;
}
const disabledSelectStyles: CSSProperties = {
  background: "rgb(192, 192, 192)",
  borderColor: "rgb(192, 192, 192)",
  color: "rgba(0, 0, 0, 0.4)",
};

const selectStyle: StylesConfig<Option, false> = {
  control: (provided, state) => {
    // provided has CSSObject type
    // state has ControlProps type

    provided.borderRadius = 0;
    if (state.isDisabled) {
      return {
        ...provided,
        ...disabledSelectStyles,
      };
    }
    return provided;
  },
};

const selectTheme = (theme: Theme): Theme => ({
  ...theme,
  borderRadius: 0,
  colors: {
    ...theme.colors,
    primary: "#00e4d4",
    primary25: "#defcfa",
    primary50: "#8ffff8",
    primary75: "#3bf5e9", // TODO: precise colors calculated, not guessed
  },
});
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
    toggleInterface(props.interfaceType, formData.isEnabled);
  };

  // TODO:consider storing in backend
  const resolveGroupName = (groupPubkey: string): Option | null => {
    let group = groups.filter((group: Group) => group.group_id === groupPubkey);
    if (group.length != 1) {
      return null;
    }
    return { label: group[0].name, value: group[0].group_id };
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
        <div className={styles["form__enabled"]}>
          <Switch
            onChange={handleIsEnabledChange}
            checked={formData.isEnabled}
            disabled={!props.canBeDisabled}
            onColor={"#00e4d4"} // TODO: global color definition
            boxShadow="0px 1px 5px rgba(0, 0, 0, 0.6)"
            activeBoxShadow="0px 0px 1px 10px rgba(0, 0, 0, 0.2)"
          />
        </div>
        <div className={styles["form__interface_name"]}>
          <h2>{props.displayName}</h2>
        </div>
        <Creatable
          maxMenuHeight={130}
          isDisabled={!formData.isEnabled}
          className={styles["form__communicator_input"]}
          value={optionOrNull(formData.communicatorUrl)}
          onChange={handleCommunicatorUrlChange}
          onCreateOption={handleCommunicatorUrlCreation}
          name="communicatorUrl"
          options={options as any}
          placeholder="Select an option"
          styles={selectStyle}
          theme={selectTheme}
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
          styles={selectStyle}
          placeholder="Select an option"
          className={styles["form__select_pubkey"]}
          isDisabled={!formData.isEnabled || !formData.communicatorUrl}
          onChange={handleGroupChange}
          components={{ Option: MultilineSelectOption }}
          value={resolveGroupName(formData["selectedGroup"])}
          theme={selectTheme}
          maxMenuHeight={120}
        />
        <label className={styles["form__select_pubkey_label"]}>Group</label>
        <button onClick={saveConfiguration} className={styles["form__apply"]}>
          Apply
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

/**
 * If the option is falsy, e.g., "" we need to pass a falsy value to react-select
 * or else the placeholder won't be displayed
 */
const optionOrNull = (value: string | null): Option | null => {
  if (!value) {
    // value may == ""
    return null;
  }
  return createOption(value);
};

const toggleInterface = (
  interfaceType: CryptographicInterface,
  isEnabled: boolean
) => {
  if (!isCreatableInterface(interfaceType)) {
    return;
  }
  const creatableInterface = interfaceType as CreatableInterface;
  if (isEnabled) {
    spawnInterfaceProcess(creatableInterface);
  } else {
    killInterfaceProcess(creatableInterface);
  }
};

const isCreatableInterface = (
  interfaceType: CryptographicInterface
): interfaceType is CreatableInterface =>
  interfaceType == "webauthn" || interfaceType == "pcsc";
