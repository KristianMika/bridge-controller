import styles from "./InterfaceConfiguration.module.css";
import Switch from "react-switch";
import "react-dropdown/style.css";
import Creatable from "react-select/creatable";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import React, { useEffect, useState } from "react";
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
import Select from "react-select";
import MultilineSelectOption from "./MultilineSelectOption/MultilineSelectOption";
import CertificateUpload from "./CertificateUpload";
import IInterfaceForm, { defaultFormData } from "../../models/IInterfaceForm";
import IInterfaceConfiguration from "../../models/IInterfaceConfiguration";
import IMultiLabelOptionType from "../../models/IMultiLabelOptionType";
import shortenHexString from "../../utils";
import { selectTheme, primaryColor, selectStyle } from "../../themes";
import IOption from "../../models/IOption";

const HEX_PUBKEY_DISPLAY_CHARS_COUNT = 10;
const DEFAULT_COMMUNICATOR_URLS = ["meesign.crocs.fi.muni.cz", "localhost"];

const createOption = (option: string): IOption => {
  return { value: option, label: option };
};

const createOptions = (options: string[]): IOption[] => {
  return options.map(createOption);
};

const InterfaceConfiguration: React.FC<IInterfaceConfiguration> = (props) => {
  const [formData, setFormData] = useState<IInterfaceForm>(() => {
    return { ...defaultFormData, isEnabled: !props.canBeDisabled };
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

  const setCommunicatorUrl = (url: string) => {
    setFormData((prev) => {
      return { ...prev, communicatorUrl: url };
    });
  };

  const handleGroupChange = (event: IMultiLabelOptionType) => {
    setFormData((prev: IInterfaceForm) => {
      return { ...prev, selectedGroup: event.value };
    });
  };

  const loadFormData = async () => {
    let configuration = await getInterfaceConfiguration(
      props.interfaceType,
      props.tool.tool
    );
    if (!configuration) {
      return;
    }
    let isCertificatePresentPromise = isCertificatePresent(
      configuration.communicatorUrl
    );
    setFormData(configuration);

    let certPresent = await isCertificatePresentPromise;
    setIsCertUploaded(certPresent);

    loadGroups(configuration.communicatorUrl);
  };
  useEffect(() => {
    loadFormData().catch((_err) => {});
  }, [props.tool, props.interfaceType]);

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
    let tool = props.tool.tool;
    setInterfaceConfiguration(props.interfaceType, tool, formData)
      .then(() => toast.success("Configuration saved"))
      .catch(() => toast.error("Failed to save configuration"));
    toggleInterface(props.interfaceType, formData.isEnabled);
  };

  const resolveGroupName = (groupPubkey: string): IOption | null => {
    if (groupPubkey.length == 0) {
      return null;
    }
    let group = groups.filter((group: Group) => group.group_id === groupPubkey);
    if (group.length != 1) {
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
    if (!(await loadCertPresent(communicatorUrl))) {
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
    <>
      <div className={styles["interface-configuration"]}>
        <form className={styles["interface-configuration__form"]}>
          <div className={styles["form--enabled"]}>
            <Switch
              onChange={handleIsEnabledChange}
              checked={formData.isEnabled}
              disabled={!props.canBeDisabled}
              onColor={primaryColor}
              boxShadow="0px 1px 5px rgba(0, 0, 0, 0.6)"
              activeBoxShadow="0px 0px 1px 10px rgba(0, 0, 0, 0.2)"
            />
          </div>
          <div className={styles["form__interface-name"]}>
            <h2>{props.displayName}</h2>
          </div>
          <Creatable
            maxMenuHeight={130}
            isDisabled={!formData.isEnabled}
            className={styles["form__communicator-input"]}
            value={optionOrNull(formData.communicatorUrl)}
            onChange={handleCommunicatorUrlChange}
            onCreateOption={handleCommunicatorUrlCreation}
            name="communicatorUrl"
            options={options as any}
            placeholder="Select an option"
            styles={selectStyle}
            theme={selectTheme}
          ></Creatable>

          <label className={styles["form__communicator-input-label"]}>
            Communicator URL
          </label>
          <CertificateUpload
            className={styles["form__communicator-file-upload-button"]}
            isDisabled={!formData.isEnabled || !formData.communicatorUrl}
            communicatorUrl={formData.communicatorUrl}
            isUploaded={isCertUploaded}
            setIsUploaded={setIsCertUploaded}
          />

          <label
            className={styles["form__communicator-file-upload-button-label"]}
          >
            Communicator Cert
          </label>
          <Select
            options={groups.map((group) => {
              return {
                label: group.name,
                subLabel: shortenHexString(
                  group.group_id,
                  HEX_PUBKEY_DISPLAY_CHARS_COUNT
                ),
                value: group.group_id,
              };
            })}
            styles={selectStyle}
            placeholder="Select a group"
            className={styles["form__select-pubkey"]}
            isDisabled={
              !formData.isEnabled ||
              !formData.communicatorUrl ||
              !isCertUploaded
            }
            onChange={handleGroupChange}
            components={{ Option: MultilineSelectOption }}
            value={resolveGroupName(formData["selectedGroup"])}
            theme={selectTheme}
            maxMenuHeight={120}
            noOptionsMessage={() => "No groups found"}
          />
          <label className={styles["form__select-pubkey-label"]}>Group</label>
          <button onClick={saveConfiguration} className={styles["form__apply"]}>
            Apply
          </button>
          <ToastContainer
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
    </>
  );
};

/**
 * If the option is falsy, e.g., "" we need to pass a falsy value to react-select
 * or else the placeholder won't be displayed
 */
const optionOrNull = (value: string | null): IOption | null => {
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

export default InterfaceConfiguration;
