## evidence_id

ev_fc768850e6c2eb2221afe4fe207776a6d64049ff701c221b5f0cf47247237046

## subject.name

LightBurn.exe

## subject.digest

| Key    | Value                                                            |
|--------|------------------------------------------------------------------|
| sha256 | ad094c3c6ab5c0a315e0dd1eb6b587619b87e037de11b1385c1248411f71ce89 |

## subject.local_path

/re/LightBurn.exe

## subject.format

pe

## subject.architecture

x86_64

## provider

| Key     | Value  |
|---------|--------|
| id      | ghidra |
| name    | Ghidra |
| version | 12.1.4 |

## analysis_profile.provider

| Key     | Value  |
|---------|--------|
| id      | ghidra |
| name    | Ghidra |
| version | 12.1.4 |

## analysis_profile.parameters.target_kind

executable

## analysis_profile.parameters.target_format

pe

## analysis_profile.parameters.executable_role

application

## analysis_profile.parameters.managed

false

## analysis_profile.parameters.native_aot_metadata

| Key                  | Value                       |
|----------------------|-----------------------------|
| contract_revision    | rtr-9.1-x64-pe-read-only-v5 |
| source_bytes         | 134217728                   |
| work_units           | 67108864                    |
| working_memory_bytes | 67108864                    |
| report_bytes         | 4194304                     |

## analysis_profile.parameters.architecture

x86_64

## analysis_profile.parameters.available_architectures

x86_64

## analysis_profile.parameters.import_mode

ephemeral-source-immutable

## analysis_profile.parameters.pdb_observations

program-info-v5

## analysis_profile.parameters.annotation_policy

atomic-function-entry-metadata-v1

## analysis_profile.parameters.load_image_observations

source-mappings-entry-context-v2

## analysis_profile.parameters.function_body_evidence

complete-inclusive-ranges-v1

## analysis_profile.parameters.function_references

complete-body-and-entry-reference-manager-v2

## analysis_profile.parameters.call_relationship_coverage

function-body-call-coverage-v5

## analysis_profile.parameters.function_dossier_coverage

observed-facets-and-call-relationships-v1

## analysis_profile.parameters.string_inventory_evidence

defined-data-coverage-v1

## analysis_profile.parameters.location_resolution

explicit-address-exact-entry-symbol-first-v3

## analysis_profile.parameters.procedure_file_offset_resolution

address-first-unique-target-source-mapping-v2

## analysis_profile.parameters.instruction_flow_evidence

decoded-return-pcode-v1

## analysis_profile.parameters.instruction_references

exact-address-incoming-without-synthetic-entry-v2

## analysis_profile.parameters.function_boundary_observations

ghidra-function-boundary-limitations-v4

## analysis_profile.parameters.no_return_repair

returning-imports-decoded-return-v2

## analysis_profile.parameters.process_launch

inspected-jvm-launch-support-v1

## analysis_profile.parameters.jump_table_evidence

typed-case-default-blocks-v1

## analysis_profile.parameters.decompiler_jump_loads

true

## analysis_profile.parameters.loader

auto-from-header

## analysis_profile.parameters.language_id

auto-from-header

## analysis_profile.parameters.compiler_spec_id

auto-default

## analysis_profile.parameters.analyzer_preset

ghidra-default

## analysis_profile.digest

8deff58aed9d509ca1f5e324742162509fc546437bff519805a5e5497ec5f789

## predicate_type

rea.analysis

## operation

procedure_pseudo_code

## parameters

| Key       | Value       |
|-----------|-------------|
| procedure | 0x140578c34 |

## raw_result.value


void FUN_1405783a0(void)

{
  int iVar1;
  QString *pQVar2;
  QString *pQVar3;
  int **local_res8;
  QArrayData *local_118;
  wchar_t *local_110;
  undefined8 local_108;
  QString local_100 [24];
  QString local_e8 [24];
  QString local_d0 [24];
  undefined4 local_b8;
  code *local_b0;
  QString local_a8 [24];
  QString local_90 [24];
  QString local_78 [24];
  QString local_60 [24];
  QString local_48 [24];
  QString local_30 [24];
  
  QString::QString(local_78,":/res/icons/ethernet-off-128.png");
  QString::QString(local_60,":/res/icons/usb-off-128.png");
  QString::QString(local_30,":/res/icons/lb_bridge-128.png");
  QString::QString(local_48,":/res/icons/ban-128.png");
  pQVar2 = (QString *)QCoreApplication::translate((char *)local_90,"Connection","Packet/USB",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"CDM";
  local_108 = 3;
  pQVar3 = (QString *)QString::QString(local_a8,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_60);
  local_b8 = 1;
  local_b0 = FUN_140533f50;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_a8);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_90);
  pQVar2 = (QString *)QCoreApplication::translate((char *)local_a8,"Connection","USB",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"LibUSB";
  local_108 = 6;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_60);
  local_b8 = 6;
  local_b0 = FUN_140534880;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)
           QCoreApplication::translate((char *)local_a8,"Connection","Ethernet or Wifi/TCP",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"TCP";
  local_108 = 3;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_78);
  local_b8 = 3;
  local_b0 = FUN_140535280;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)
           QCoreApplication::translate((char *)local_a8,"Connection","Ethernet or Wifi/UDP",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"UDP";
  local_108 = 3;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_78);
  local_b8 = 2;
  local_b0 = FUN_140535750;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)QCoreApplication::translate((char *)local_a8,"Connection","Serial/USB",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"Serial";
  local_108 = 6;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_60);
  local_b8 = 0;
  local_b0 = FUN_140534960;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)
           QCoreApplication::translate((char *)local_a8,"Connection","Ethernet or Wifi/HTTP",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"HTTP";
  local_108 = 4;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_78);
  local_b8 = 9;
  local_b0 = FUN_140534850;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)
           QCoreApplication::translate((char *)local_a8,"Connection","LightBurn Bridge",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"TCP_R";
  local_108 = 5;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_30);
  local_b8 = 4;
  local_b0 = FUN_1405352e0;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)
           QCoreApplication::translate((char *)local_a8,"Connection","Ethernet or Wifi/TCP Ruida",0)
  ;
  local_118 = (QArrayData *)0x0;
  local_110 = L"TCP_PK";
  local_108 = 6;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_78);
  local_b8 = 5;
  local_b0 = FUN_1405352b0;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)
           QCoreApplication::translate((char *)local_a8,"Connection","Ethernet or Wifi/WebAPI",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"WebApi";
  local_108 = 6;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_78);
  local_b8 = 0xc;
  local_b0 = FUN_1405357b0;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)
           QCoreApplication::translate((char *)local_a8,"Connection","Ethernet or Wifi",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"Epilog";
  local_108 = 6;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_78);
  local_b8 = 7;
  local_b0 = FUN_1405340d0;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)QCoreApplication::translate((char *)local_a8,"Connection","None",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"None";
  local_108 = 4;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_48);
  local_b8 = 0xd;
  local_b0 = FUN_140534920;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  QString::~QString(local_48);
  QString::~QString(local_30);
  QString::~QString(local_60);
  QString::~QString(local_78);
  return;
}



## raw_result.limitations



## normalized_result


void FUN_1405783a0(void)

{
  int iVar1;
  QString *pQVar2;
  QString *pQVar3;
  int **local_res8;
  QArrayData *local_118;
  wchar_t *local_110;
  undefined8 local_108;
  QString local_100 [24];
  QString local_e8 [24];
  QString local_d0 [24];
  undefined4 local_b8;
  code *local_b0;
  QString local_a8 [24];
  QString local_90 [24];
  QString local_78 [24];
  QString local_60 [24];
  QString local_48 [24];
  QString local_30 [24];
  
  QString::QString(local_78,":/res/icons/ethernet-off-128.png");
  QString::QString(local_60,":/res/icons/usb-off-128.png");
  QString::QString(local_30,":/res/icons/lb_bridge-128.png");
  QString::QString(local_48,":/res/icons/ban-128.png");
  pQVar2 = (QString *)QCoreApplication::translate((char *)local_90,"Connection","Packet/USB",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"CDM";
  local_108 = 3;
  pQVar3 = (QString *)QString::QString(local_a8,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_60);
  local_b8 = 1;
  local_b0 = FUN_140533f50;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_a8);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_90);
  pQVar2 = (QString *)QCoreApplication::translate((char *)local_a8,"Connection","USB",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"LibUSB";
  local_108 = 6;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_60);
  local_b8 = 6;
  local_b0 = FUN_140534880;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)
           QCoreApplication::translate((char *)local_a8,"Connection","Ethernet or Wifi/TCP",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"TCP";
  local_108 = 3;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_78);
  local_b8 = 3;
  local_b0 = FUN_140535280;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)
           QCoreApplication::translate((char *)local_a8,"Connection","Ethernet or Wifi/UDP",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"UDP";
  local_108 = 3;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_78);
  local_b8 = 2;
  local_b0 = FUN_140535750;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)QCoreApplication::translate((char *)local_a8,"Connection","Serial/USB",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"Serial";
  local_108 = 6;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_60);
  local_b8 = 0;
  local_b0 = FUN_140534960;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)
           QCoreApplication::translate((char *)local_a8,"Connection","Ethernet or Wifi/HTTP",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"HTTP";
  local_108 = 4;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_78);
  local_b8 = 9;
  local_b0 = FUN_140534850;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)
           QCoreApplication::translate((char *)local_a8,"Connection","LightBurn Bridge",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"TCP_R";
  local_108 = 5;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_30);
  local_b8 = 4;
  local_b0 = FUN_1405352e0;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)
           QCoreApplication::translate((char *)local_a8,"Connection","Ethernet or Wifi/TCP Ruida",0)
  ;
  local_118 = (QArrayData *)0x0;
  local_110 = L"TCP_PK";
  local_108 = 6;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_78);
  local_b8 = 5;
  local_b0 = FUN_1405352b0;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)
           QCoreApplication::translate((char *)local_a8,"Connection","Ethernet or Wifi/WebAPI",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"WebApi";
  local_108 = 6;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_78);
  local_b8 = 0xc;
  local_b0 = FUN_1405357b0;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)
           QCoreApplication::translate((char *)local_a8,"Connection","Ethernet or Wifi",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"Epilog";
  local_108 = 6;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_78);
  local_b8 = 7;
  local_b0 = FUN_1405340d0;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  pQVar2 = (QString *)QCoreApplication::translate((char *)local_a8,"Connection","None",0);
  local_118 = (QArrayData *)0x0;
  local_110 = L"None";
  local_108 = 4;
  pQVar3 = (QString *)QString::QString(local_90,(QArrayDataPointer<char16_t> *)&local_118);
  QString::QString(local_100,pQVar3);
  QString::QString(local_e8,pQVar2);
  QString::QString(local_d0,local_48);
  local_b8 = 0xd;
  local_b0 = FUN_140534920;
  local_res8 = &DAT_141f5b578;
  FUN_140519870(&local_res8,DAT_141f5b588,local_100);
  if ((DAT_141f5b578 == (int *)0x0) || (1 < *DAT_141f5b578)) {
    FUN_1405b1650(&DAT_141f5b578,0,0,0);
  }
  QString::~QString(local_d0);
  QString::~QString(local_e8);
  QString::~QString(local_100);
  QString::~QString(local_90);
  if (local_118 != (QArrayData *)0x0) {
    LOCK();
    iVar1 = *(int *)local_118;
    *(int *)local_118 = *(int *)local_118 + -1;
    UNLOCK();
    if (iVar1 == 1) {
      QArrayData::deallocate(local_118,2,8);
    }
  }
  QString::~QString(local_a8);
  QString::~QString(local_48);
  QString::~QString(local_30);
  QString::~QString(local_60);
  QString::~QString(local_78);
  return;
}



## confidence

observed

## authority

shipped-artifact

## environment

null

## limitations

The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.,The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.,Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.,Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.,Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.,External functions and functions without an analyzable body return null; other decompiler failures remain explicit.,The Ghidra Program references PDB LightBurn.pdb (GUID 3ea7c6e8-2b80-4239-83f1-69b559de8ff9, age (hex) 1, signature unknown, version RSDS); Ghidra reports load state unknown. Original names and types may be missing; an empty name search does not prove a procedure is absent. Use list_procedures and address-based analysis to inspect recovered functions. REA does not admit a caller-selected PDB companion.

## relationship_coverage

null

## locations



## evidence_links

