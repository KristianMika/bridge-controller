interface IInterfaceForm {
  isEnabled: boolean;
  communicatorUrl: string;
  selectedGroup: string;
}

export const defaultFormData: IInterfaceForm = {
  isEnabled: false,
  communicatorUrl: "",
  selectedGroup: "",
};

export default IInterfaceForm;
