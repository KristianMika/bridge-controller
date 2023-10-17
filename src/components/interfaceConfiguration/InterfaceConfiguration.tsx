import styles from "./InterfaceConfiguration.module.css";
import Switch from "react-switch";
import "react-dropdown/style.css";
import Creatable from "react-select/creatable";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
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
  isCertificatePresent,
} from "../../bindings";
import Select, { StylesConfig } from "react-select";
import { MultilineSelectOption } from "./MultilineSelectOption/MultilineSelectOption";
import { CertificateUpload } from "./CertificateUpload";
import IInterfaceForm, { defaultFormData } from "../../models/IInterfaceForm";
import IInterfaceConfiguration from "../../models/IInterfaceConfiguration";
import IOptionType from "../../models/IOptionType";
import shortenHexPubkey from "../../utils";
import selectTheme from "../../themes";

const HEX_PUBKEY_DISPLAY_CHARS_COUNT = 10;

const DEFAULT_COMMUNICATOR_URLS = ["meesign.crocs.fi.muni.cz", "localhost"];

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

const createOption = (option: string): Option => {
  return { value: option, label: option };
};

const createOptions = (options: string[]): Option[] => {
  return options.map(createOption);
};

export const InterfaceConfiguration: React.FC<IInterfaceConfiguration> = (
  props
) => {
  const [formData, setFormData] = useState<IInterfaceForm>(() => {
    return { ...defaultFormData };
  });
  const [groups, setGroups] = useState<Group[]>([]);
  const [isCertUploaded, setIsCertUploaded] = useState<boolean>(false);
  const [options, setOptions] = useState(
    createOptions(DEFAULT_COMMUNICATOR_URLS)
  );

  const handleIsEnabledChange = (checked: boolean) => {
    setFormData((prev: IInterfaceForm) => {
      return { ...prev, isEnabled: checked };
    });
  };
  const setSelectedGroup = (group: string) => {
    setFormData((prev: IInterfaceForm) => {
      return { ...prev, selectedGroup: group };
    });
  };
  const setCommunicatorUrl = (url: string) => {
    setFormData((prev) => {
      return { ...prev, communicatorUrl: url };
    });
  };

  const handleGroupChange = (event: IOptionType) => {
    setFormData((prev: IInterfaceForm) => {
      return { ...prev, selectedGroup: event.value };
    });
  };

  const loadFormData = async () => {
    let configuration = await getInterfaceConfiguration(props.interfaceType);
    if (!configuration) {
      return;
    }
    let isCertificatePresentPromise = isCertificatePresent(
      configuration.communicatorUrl
    );
    setFormData(configuration);

    let certUploaded = await isCertificatePresentPromise;
    setIsCertUploaded(certUploaded);

    // groups are loaded as a side effect of communicator url change
  };
  useEffect(() => {
    loadFormData().catch((_err) => {});
  }, []);

  const handleCommunicatorUrlCreation = (inputValue: string) => {
    setCommunicatorUrl(inputValue);
    setOptions((prev) => [...prev, createOption(inputValue)]);
  };

  const isConfigurationValidWithSideEffects = (): boolean => {
    if (!formData.communicatorUrl) {
      toast.error("Communicator URL is not set");
      return false;
    }
    if (formData.selectedGroup.length == 0) {
      toast.error("Group is not set");
      return false;
    }

    if (!isCertUploaded) {
      toast.error("Missing certificate");
      return false;
    }
    return true;
  };
  const saveConfiguration = (event: React.MouseEvent<HTMLElement>) => {
    event.preventDefault();
    if (!isConfigurationValidWithSideEffects()) {
      return;
    }
    setInterfaceConfiguration(props.interfaceType, formData);
    toggleInterface(props.interfaceType, formData.isEnabled);
  };

  const resolveGroupName = (groupPubkey: string): Option | null => {
    if (groupPubkey.length == 0) {
      return null;
    }
    let group = groups.filter((group: Group) => group.group_id === groupPubkey);
    if (group.length != 1) {
      setSelectedGroup("");
      return null;
    }
    return { label: group[0].name, value: group[0].group_id };
  };

  const loadCertPresent = async (communicatorUrl: string) => {
    let certPresent = await isCertificatePresent(communicatorUrl);
    setIsCertUploaded(certPresent);
    return certPresent;
  };

  const loadGroups = async (communicatorUrl: string) => {
    if (!loadCertPresent) {
      return;
    }

    try {
      let groups = await getGroups(communicatorUrl);
      setGroups(groups);
    } catch (_err) {
      toast.error(`Failed to fetch groups from "${communicatorUrl}"`);
    }
  };

  const handleCommunicatorUrlChange = (newValue: any) => {
    setGroups([]);
    setFormData((prev) => {
      return { ...prev, selectedGroup: "" };
    });
    setCommunicatorUrl(newValue.value);
    loadGroups(newValue.value);
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
          isUploaded={isCertUploaded}
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
              subLabel: shortenHexPubkey(
                group.group_id,
                HEX_PUBKEY_DISPLAY_CHARS_COUNT
              ),
              value: group.group_id,
            };
          })}
          styles={selectStyle}
          placeholder="Select an option"
          className={styles["form__select_pubkey"]}
          isDisabled={
            !formData.isEnabled || !formData.communicatorUrl || !isCertUploaded
          }
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
        <ToastContainer
          className={styles["toast-position"]}
          position="top-right"
          autoClose={2000}
          hideProgressBar={false}
          newestOnTop={false}
          closeOnClick
          rtl={false}
          pauseOnFocusLoss
          draggable
          pauseOnHover
          theme="dark"
        />
      </form>
    </div>
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
