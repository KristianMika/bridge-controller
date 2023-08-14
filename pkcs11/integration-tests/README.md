# Integration tests

## Softfido Test Suite with Meesign Cryptoki

### Install requirements

- unfortunately, this test suite can't be run in CI/CD, as Github runners don't contain _vhci-hcd_ kernel module.

```
sudo apt-get install opensc
pip3 install fido2==0.7
```

### Run tests

```
./run_softfido_tests.sh
```
