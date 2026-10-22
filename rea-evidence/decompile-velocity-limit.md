## evidence_id

ev_a87f9657ea71bae2d46a781f7962deaadcbeb0caec5ea1080dc64ef3168c6bd0

## subject.name

acclplanner.dll

## subject.digest

| Key    | Value                                                            |
|--------|------------------------------------------------------------------|
| sha256 | c1809a53cc2334b11bfcd5681d7975b7b4fcc3e2f0462dd1fe5316ec2cd9d460 |

## subject.local_path

C:\Program Files\LightBurn\acclplanner.dll

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

shared-library

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

disabled-windows-p0

## analysis_profile.parameters.process_launch

official-headless-script-v1

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

189821c22f9a3c65d594fc3eda7b5a03806941e5d6504c95c1b268b890a7c7ff

## predicate_type

rea.analysis

## operation

procedure_pseudo_code

## parameters

| Key       | Value       |
|-----------|-------------|
| procedure | 0x180037456 |

## raw_result.value


/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Type propagation algorithm not settling */

undefined8 * FUN_1800365d0(undefined8 *param_1,longlong *param_2,char param_3)

{
  ulonglong uVar1;
  int iVar2;
  char cVar3;
  char cVar4;
  int iVar5;
  undefined8 uVar6;
  char **ppcVar7;
  undefined8 *puVar8;
  ulonglong *puVar9;
  undefined8 uVar10;
  undefined8 *puVar11;
  char **ppcVar12;
  undefined8 ******ppppppuVar13;
  void *pvVar14;
  undefined8 ******ppppppuVar15;
  longlong lVar16;
  undefined8 *******_Src;
  size_t _Size;
  undefined8 *******pppppppuVar17;
  ulonglong *_Src_00;
  ulonglong *_Size_00;
  bool bVar18;
  bool bVar19;
  char *pcVar20;
  undefined1 auStack_1d8 [32];
  char *local_1b8;
  undefined8 local_1b0;
  char local_1a8 [4];
  undefined4 local_1a4;
  char local_1a0;
  char *local_198 [2];
  char *local_188;
  char *local_180;
  undefined8 *local_178;
  void *local_170;
  undefined8 uStack_168;
  undefined8 local_160;
  ulonglong uStack_158;
  char *local_150;
  undefined8 uStack_148;
  undefined8 local_140;
  ulonglong uStack_138;
  undefined8 ******local_130;
  undefined8 ******ppppppuStack_128;
  undefined8 ******local_120;
  undefined8 ******ppppppuStack_118;
  void *local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  ulonglong uStack_f8;
  undefined8 *******local_f0;
  undefined8 uStack_e8;
  longlong local_e0;
  ulonglong uStack_d8;
  undefined1 local_d0 [32];
  undefined1 local_b0 [32];
  undefined1 local_90 [32];
  ulonglong local_70;
  
  local_70 = DAT_1803386c0 ^ (ulonglong)auStack_1d8;
  *param_1 = 0;
  param_1[1] = 0;
  param_1[2] = 0;
  local_1a4 = 1;
  lVar16 = -1;
  local_198[0] = (char *)CONCAT44(local_198[0]._4_4_,0xffffffff);
  local_160 = 8;
  uStack_158 = 0xf;
  local_170 = (void *)0x64695f726573616c;
  uStack_168 = 0;
  local_1a0 = param_3;
  local_178 = param_1;
  cVar3 = FUN_180055f90(param_2,&local_170);
  if (cVar3 == '\0') {
    if (*param_2 == 0) {
      iVar5 = (int)local_198[0];
    }
    else {
      iVar5 = FUN_180023c70(*param_2,&local_170,local_198);
    }
  }
  else {
    iVar5 = FUN_180023b10(param_2,&local_170);
  }
  if (0xf < uStack_158) {
    pvVar14 = local_170;
    if ((0xfff < uStack_158 + 1) &&
       (pvVar14 = *(void **)((longlong)local_170 + -8),
       0x1f < (ulonglong)((longlong)local_170 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pvVar14);
  }
  if (iVar5 == 1) {
    local_1a8[0] = '\0';
    local_160 = 0xc;
    uStack_158 = 0xf;
    local_170 = (void *)0x616e655f61706f6d;
    uStack_168 = 0x64656c62;
    cVar3 = FUN_180055f90(param_2,&local_170);
    if (cVar3 == '\0') {
      cVar3 = local_1a8[0];
      if (*param_2 != 0) {
        cVar3 = FUN_180025360(*param_2,&local_170,local_1a8);
      }
    }
    else {
      cVar3 = FUN_180025200(param_2,&local_170);
    }
    if (0xf < uStack_158) {
      FUN_180019d40(&local_170,local_170);
    }
    local_198[0] = (char *)((ulonglong)local_198[0] & 0xffffffff00000000);
    local_150 = (char *)0x0;
    uStack_148 = 0;
    local_140 = 0;
    uStack_138 = 0;
    FUN_1800155c0(&local_150,"mopa_pulse_width",0x10);
    cVar4 = FUN_180055f90(param_2,&local_150);
    if (cVar4 == '\0') {
      if (*param_2 == 0) {
        iVar5 = (int)local_198[0];
      }
      else {
        iVar5 = FUN_180023c70(*param_2,&local_150,local_198);
      }
    }
    else {
      iVar5 = FUN_180023b10(param_2,&local_150);
    }
    if (0xf < uStack_138) {
      pcVar20 = local_150;
      if ((0xfff < uStack_138 + 1) &&
         (pcVar20 = *(char **)(local_150 + -8), (char *)0x1f < local_150 + (-8 - (longlong)pcVar20))
         ) {
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(pcVar20);
    }
    if ((cVar3 != '\0') && (0 < iVar5)) {
      uStack_148 = 0;
      local_140 = 0;
      uStack_138 = 0xf;
      local_150 = (char *)0x0;
      local_1a4 = 0x201;
      FUN_180029cd0(&local_150,(double)iVar5);
      uVar6 = FUN_18001ffb0(&local_110,"MOPA_SET_PULSE_WIDTH DURATION=",&local_150);
      FUN_1800239a0(param_1,uVar6);
      FUN_180018de0(&local_110);
      local_1a4 = 1;
      if (0xf < uStack_138) {
        FUN_180019d40(&local_150,local_150);
      }
    }
    local_198[0] = (char *)((ulonglong)local_198[0] & 0xffffffff00000000);
    local_160 = 9;
    uStack_158 = 0xf;
    local_170 = (void *)0x6572665f61706f6d;
    uStack_168 = 0x71;
    cVar4 = FUN_180055f90(param_2,&local_170);
    if (cVar4 == '\0') {
      if (*param_2 == 0) {
        iVar5 = (int)local_198[0];
      }
      else {
        iVar5 = FUN_180023c70(*param_2,&local_170,local_198);
      }
    }
    else {
      iVar5 = FUN_180023b10(param_2,&local_170);
    }
    local_188 = (char *)CONCAT44(local_188._4_4_,iVar5);
    if (0xf < uStack_158) {
      pvVar14 = local_170;
      if ((0xfff < uStack_158 + 1) &&
         (pvVar14 = *(void **)((longlong)local_170 + -8),
         0x1f < (ulonglong)((longlong)local_170 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(pvVar14);
    }
    if (0 < iVar5) {
      ppcVar7 = &local_188;
      if (cVar3 == '\0') {
        if (iVar5 < 0x1e) {
          ppcVar7 = (char **)&DAT_18028ec40;
        }
        ppcVar12 = (char **)&DAT_18028ec44;
        bVar19 = SBORROW4(iVar5,0x3c);
        iVar2 = -0x3c;
        bVar18 = iVar5 == 0x3c;
      }
      else {
        ppcVar12 = (char **)&DAT_18028ec3c;
        bVar19 = SBORROW4(iVar5,4000);
        iVar2 = -4000;
        bVar18 = iVar5 == 4000;
      }
      if (!bVar18 && bVar19 == iVar5 + iVar2 < 0) {
        ppcVar7 = ppcVar12;
      }
      ppppppuStack_128 = (undefined8 ******)0x0;
      local_120 = (undefined8 ******)0x0;
      ppppppuStack_118 = (undefined8 ******)0xf;
      local_130 = (undefined8 ******)0x0;
      local_1a4 = 0x101;
      FUN_180029cd0(&local_130,(double)*(int *)ppcVar7);
      uVar6 = FUN_18001ffb0(&local_110,"MOPA_SET_FREQ FREQ=",&local_130);
      FUN_1800239a0(param_1,uVar6);
      FUN_180018de0(&local_110);
      local_1a4 = 1;
      FUN_180018de0(&local_130);
    }
  }
  local_1a8[0] = '\0';
  local_160 = 0xe;
  uStack_158 = 0xf;
  local_170 = (void *)0x5f776f6c66726961;
  uStack_168 = 0x656c7a7a6f6e;
  cVar3 = FUN_180055f90(param_2,&local_170);
  if (cVar3 == '\0') {
    cVar3 = local_1a8[0];
    if (*param_2 != 0) {
      cVar3 = FUN_180025360(*param_2,&local_170,local_1a8);
    }
  }
  else {
    cVar3 = FUN_180025200(param_2,&local_170);
  }
  local_198[1] = "AF_SWITCH ATTACHED=0";
  if (cVar3 != '\0') {
    local_198[1] = "AF_SWITCH ATTACHED=1";
  }
  puVar11 = (undefined8 *)param_1[1];
  if (puVar11 == (undefined8 *)param_1[2]) {
    FUN_180021670(param_1,puVar11,local_198 + 1);
  }
  else {
    *puVar11 = 0;
    puVar11[1] = 0;
    puVar11[2] = 0;
    puVar11[3] = 0;
    do {
      lVar16 = lVar16 + 1;
    } while (local_198[1][lVar16] != '\0');
    FUN_1800155c0(puVar11,local_198[1],lVar16);
    param_1[1] = param_1[1] + 0x20;
  }
  pppppppuVar17 = (undefined8 *******)0x0;
  if (0xf < uStack_158) {
    pvVar14 = local_170;
    if ((0xfff < uStack_158 + 1) &&
       (pvVar14 = *(void **)((longlong)local_170 + -8),
       0x1f < (ulonglong)((longlong)local_170 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pvVar14);
  }
  local_198[0] = (char *)0x4014000000000000;
  local_198[1] = (char *)0xc014000000000000;
  local_180 = (char *)0x0;
  local_160 = 0xd;
  uStack_158 = 0xf;
  local_170 = (void *)0x65645f726573616c;
  uStack_168 = 0x7375636f66;
  cVar3 = FUN_180055f90(param_2,&local_170);
  if (cVar3 == '\0') {
    pcVar20 = local_180;
    if (*param_2 != 0) {
      pcVar20 = (char *)FUN_1800240d0(*param_2,&local_170,&local_180);
    }
  }
  else {
    pcVar20 = (char *)FUN_180023f50(param_2,&local_170);
  }
  local_188 = pcVar20;
  ppcVar7 = local_198 + 1;
  if (-5.0 <= (double)pcVar20) {
    ppcVar7 = &local_188;
  }
  ppcVar12 = local_198;
  if ((double)pcVar20 <= 5.0) {
    ppcVar12 = ppcVar7;
  }
  uStack_148 = 0;
  local_140 = 0;
  uStack_138 = 0xf;
  local_150 = (char *)0x0;
  local_1a4 = 0x21;
  FUN_180029cd0(&local_150,*ppcVar12);
  puVar8 = (undefined8 *)FUN_18001ffb0(&local_130,"LASER_DEFOCUS DISTANCE=",&local_150);
  puVar11 = (undefined8 *)param_1[1];
  if (puVar11 == (undefined8 *)param_1[2]) {
    FUN_1800164a0(param_1,puVar11,puVar8);
  }
  else {
    *puVar11 = 0;
    puVar11[1] = 0;
    puVar11[2] = 0;
    puVar11[3] = 0;
    uVar6 = puVar8[1];
    *puVar11 = *puVar8;
    puVar11[1] = uVar6;
    uVar6 = puVar8[3];
    puVar11[2] = puVar8[2];
    puVar11[3] = uVar6;
    puVar8[2] = 0;
    puVar8[3] = 0xf;
    *(undefined1 *)puVar8 = 0;
    param_1[1] = param_1[1] + 0x20;
  }
  if (0xf < ppppppuStack_118) {
    FUN_180019d40(&local_130,local_130);
  }
  local_120 = (undefined8 ******)0x0;
  ppppppuStack_118 = (undefined8 ******)0xf;
  local_130 = (undefined8 ******)((ulonglong)local_130 & 0xffffffffffffff00);
  local_1a4 = 1;
  if (0xf < uStack_138) {
    pcVar20 = local_150;
    if ((0xfff < uStack_138 + 1) &&
       (pcVar20 = *(char **)(local_150 + -8), (char *)0x1f < local_150 + (-8 - (longlong)pcVar20)))
    {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pcVar20);
  }
  local_140 = 0;
  uStack_138 = 0xf;
  local_150 = (char *)((ulonglong)local_150 & 0xffffffffffffff00);
  if (0xf < uStack_158) {
    pvVar14 = local_170;
    if ((0xfff < uStack_158 + 1) &&
       (pvVar14 = *(void **)((longlong)local_170 + -8),
       0x1f < (ulonglong)((longlong)local_170 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pvVar14);
  }
  local_180 = (char *)CONCAT44(local_180._4_4_,100);
  local_188 = (char *)((ulonglong)local_188 & 0xffffffff00000000);
  local_198[1] = (char *)0x0;
  local_160 = 0xf;
  uStack_158 = 0xf;
  local_170 = (void *)0x6c665f726573616c;
  uStack_168 = 0x646c6569667461;
  cVar3 = FUN_180055f90(param_2,&local_170);
  if (cVar3 == '\0') {
    pcVar20 = local_198[1];
    if (*param_2 != 0) {
      pcVar20 = (char *)FUN_1800240d0(*param_2,&local_170,local_198 + 1);
    }
  }
  else {
    pcVar20 = (char *)FUN_180023f50(param_2,&local_170);
  }
  iVar5 = (int)(double)pcVar20;
  local_198[0] = (char *)CONCAT44(local_198[0]._4_4_,iVar5);
  ppcVar7 = &local_188;
  if (-1 < iVar5) {
    ppcVar7 = local_198;
  }
  ppcVar12 = &local_180;
  if (iVar5 < 0x65) {
    ppcVar12 = ppcVar7;
  }
  uStack_e8 = 0;
  local_e0 = 0;
  uStack_d8 = 0xf;
  local_f0 = (undefined8 *******)0x0;
  local_1a4 = 5;
  FUN_180029cd0(&local_f0,(double)*(int *)ppcVar12);
  if (uStack_d8 - local_e0 < 0x12) {
    local_1b0 = 0x12;
    local_1b8 = "LASER_FLATFIELD P=";
    pppppppuVar17 = (undefined8 *******)FUN_180022ac0(&local_f0,0x12,0,0);
  }
  else {
    _Src = &local_f0;
    if (0xf < uStack_d8) {
      _Src = local_f0;
    }
    if (((undefined8 *******)0x18028ee59 < _Src) ||
       ((char *)((longlong)_Src + local_e0) < "LASER_FLATFIELD P=")) {
      pppppppuVar17 = (undefined8 *******)0x12;
    }
    else if ("LASER_FLATFIELD P=" < _Src) {
      pppppppuVar17 = _Src + -0x30051dc9;
    }
    _Size = local_e0 + 1;
    local_e0 = local_e0 + 0x12;
    memmove((char *)((longlong)_Src + 0x12),_Src,_Size);
    memcpy(_Src,"LASER_FLATFIELD P=",(size_t)pppppppuVar17);
    memcpy((char *)((longlong)_Src + (longlong)pppppppuVar17),
           (char *)((longlong)pppppppuVar17 + 0x18028ee5a),0x12 - (longlong)pppppppuVar17);
    pppppppuVar17 = &local_f0;
  }
  local_130 = *pppppppuVar17;
  ppppppuStack_128 = pppppppuVar17[1];
  local_120 = pppppppuVar17[2];
  ppppppuStack_118 = pppppppuVar17[3];
  pppppppuVar17[2] = (undefined8 ******)0x0;
  pppppppuVar17[3] = (undefined8 ******)0xf;
  *(undefined1 *)pppppppuVar17 = 0;
  local_1a4 = 0x15;
  puVar9 = (ulonglong *)param_1[1];
  if (puVar9 == (ulonglong *)param_1[2]) {
    FUN_1800164a0(param_1,puVar9,&local_130);
    ppppppuVar15 = ppppppuStack_118;
  }
  else {
    *puVar9 = (ulonglong)local_130;
    puVar9[1] = (ulonglong)ppppppuStack_128;
    puVar9[2] = (ulonglong)local_120;
    puVar9[3] = (ulonglong)ppppppuStack_118;
    local_130 = (undefined8 ******)((ulonglong)local_130 & 0xffffffffffffff00);
    param_1[1] = param_1[1] + 0x20;
    ppppppuVar15 = (undefined8 ******)0xf;
  }
  if ((undefined8 ******)0xf < ppppppuVar15) {
    ppppppuVar13 = local_130;
    if ((0xfff < (longlong)ppppppuVar15 + 1U) &&
       (ppppppuVar13 = (undefined8 ******)local_130[-1],
       0x1f < (ulonglong)((longlong)local_130 + (-8 - (longlong)ppppppuVar13)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(ppppppuVar13);
  }
  local_1a4 = 0x11;
  if (0xf < uStack_d8) {
    pppppppuVar17 = local_f0;
    if ((0xfff < uStack_d8 + 1) &&
       (pppppppuVar17 = (undefined8 *******)local_f0[-1],
       (char *)0x1f < (char *)((longlong)local_f0 + (-8 - (longlong)pppppppuVar17)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pppppppuVar17);
  }
  local_e0 = 0;
  uStack_d8 = 0xf;
  local_f0 = (undefined8 *******)((ulonglong)local_f0 & 0xffffffffffffff00);
  if (0xf < uStack_158) {
    pvVar14 = local_170;
    if ((0xfff < uStack_158 + 1) &&
       (pvVar14 = *(void **)((longlong)local_170 + -8),
       0x1f < (ulonglong)((longlong)local_170 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pvVar14);
  }
  local_188 = (char *)CONCAT44(local_188._4_4_,100);
  local_150 = (char *)0x0;
  uStack_148 = 0;
  local_140 = 0;
  uStack_138 = 0;
  local_150 = (char *)FUN_18024833c(0x20);
  local_140 = 0x10;
  uStack_138 = 0x1f;
  builtin_strncpy(local_150,"air_assist_speed",0x11);
  cVar3 = FUN_180055f90(param_2,&local_150);
  if (cVar3 == '\0') {
    if (*param_2 == 0) {
      iVar5 = (int)local_188;
    }
    else {
      iVar5 = FUN_180023c70(*param_2,&local_150,&local_188);
    }
  }
  else {
    iVar5 = FUN_180023b10(param_2,&local_150);
  }
  if (0xf < uStack_138) {
    pcVar20 = local_150;
    if ((0xfff < uStack_138 + 1) &&
       (pcVar20 = *(char **)(local_150 + -8), (char *)0x1f < local_150 + (-8 - (longlong)pcVar20)))
    {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pcVar20);
  }
  local_1a8[0] = '\x01';
  local_160 = 10;
  uStack_158 = 0xf;
  local_170 = (void *)0x697373615f726961;
  uStack_168 = 0x7473;
  local_1a4 = 0x13;
  cVar3 = FUN_180055f90(param_2,&local_170);
  if (cVar3 == '\0') {
    cVar3 = local_1a8[0];
    if (*param_2 != 0) {
      cVar3 = FUN_180025360(*param_2,&local_170,local_1a8);
    }
  }
  else {
    cVar3 = FUN_180025200(param_2,&local_170);
  }
  if ((cVar3 == '\0') || (iVar5 < 1)) {
    iVar5 = 0;
  }
  puVar9 = (ulonglong *)FUN_1800457c0(&local_110,iVar5);
  uVar1 = puVar9[2];
  if (puVar9[3] - uVar1 < 0x15) {
    local_1b0 = 0x15;
    local_1b8 = "AIR_ASSIST_SET SPEED=";
    puVar9 = (ulonglong *)FUN_180022ac0(puVar9,0x15,0,0);
  }
  else {
    puVar9[2] = uVar1 + 0x15;
    _Src_00 = puVar9;
    if (0xf < puVar9[3]) {
      _Src_00 = (ulonglong *)*puVar9;
    }
    if (((ulonglong *)0x18028ee9c < _Src_00) ||
       ((char *)((longlong)_Src_00 + uVar1) < "AIR_ASSIST_SET SPEED=")) {
      _Size_00 = (ulonglong *)0x15;
    }
    else if ("AIR_ASSIST_SET SPEED=" < _Src_00) {
      _Size_00 = _Src_00 + -0x30051dd1;
    }
    else {
      _Size_00 = (ulonglong *)0x0;
    }
    memmove((undefined1 *)((longlong)_Src_00 + 0x15),_Src_00,uVar1 + 1);
    memcpy(_Src_00,"AIR_ASSIST_SET SPEED=",(size_t)_Size_00);
    memcpy((undefined1 *)((longlong)_Src_00 + (longlong)_Size_00),
           (undefined1 *)((longlong)_Size_00 + 0x18028ee9d),0x15 - (longlong)_Size_00);
  }
  local_130 = (undefined8 ******)*puVar9;
  ppppppuStack_128 = (undefined8 ******)puVar9[1];
  local_120 = (undefined8 ******)puVar9[2];
  ppppppuStack_118 = (undefined8 ******)puVar9[3];
  puVar9[2] = 0;
  puVar9[3] = 0xf;
  *(undefined1 *)puVar9 = 0;
  local_1a4 = 0x1b;
  puVar9 = (ulonglong *)param_1[1];
  if (puVar9 == (ulonglong *)param_1[2]) {
    FUN_1800164a0(param_1,puVar9,&local_130);
    ppppppuVar15 = ppppppuStack_118;
  }
  else {
    *puVar9 = (ulonglong)local_130;
    puVar9[1] = (ulonglong)ppppppuStack_128;
    puVar9[2] = (ulonglong)local_120;
    puVar9[3] = (ulonglong)ppppppuStack_118;
    local_130 = (undefined8 ******)((ulonglong)local_130 & 0xffffffffffffff00);
    param_1[1] = param_1[1] + 0x20;
    ppppppuVar15 = (undefined8 ******)0xf;
  }
  if (0xf < ppppppuVar15) {
    ppppppuVar13 = local_130;
    if ((0xfff < (longlong)ppppppuVar15 + 1U) &&
       (ppppppuVar13 = (undefined8 ******)local_130[-1],
       0x1f < (ulonglong)((longlong)local_130 + (-8 - (longlong)ppppppuVar13)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(ppppppuVar13);
  }
  if (0xf < uStack_f8) {
    pvVar14 = local_110;
    if ((0xfff < uStack_f8 + 1) &&
       (pvVar14 = *(void **)((longlong)local_110 + -8),
       0x1f < (ulonglong)((longlong)local_110 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pvVar14);
  }
  local_100 = 0;
  uStack_f8 = 0xf;
  local_110 = (void *)((ulonglong)local_110 & 0xffffffffffffff00);
  local_1a4 = 0x19;
  if (0xf < uStack_158) {
    pvVar14 = local_170;
    if ((0xfff < uStack_158 + 1) &&
       (pvVar14 = *(void **)((longlong)local_170 + -8),
       0x1f < (ulonglong)((longlong)local_170 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pvVar14);
  }
  if (local_1a0 != '\0') {
    local_198[1] = (char *)0x408f400000000000;
    uStack_168 = 0;
    local_160 = 5;
    uStack_158 = 0xf;
    local_170 = (void *)0x6c65636361;
    cVar3 = FUN_180055f90(param_2,&local_170);
    if (cVar3 == '\0') {
      if (*param_2 != 0) {
        FUN_1800240d0(*param_2,&local_170,local_198 + 1);
      }
    }
    else {
      FUN_180023f50(param_2,&local_170);
    }
    if (0xf < uStack_158) {
      pvVar14 = local_170;
      if ((0xfff < uStack_158 + 1) &&
         (pvVar14 = *(void **)((longlong)local_170 + -8),
         0x1f < (ulonglong)((longlong)local_170 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(pvVar14);
    }
    uVar6 = FUN_180045890(&local_130);
    puVar8 = (undefined8 *)FUN_18001ffb0(&local_110,"SET_VELOCITY_LIMIT ACCEL=",uVar6);
    puVar11 = (undefined8 *)param_1[1];
    if (puVar11 == (undefined8 *)param_1[2]) {
      FUN_1800164a0(param_1,puVar11,puVar8);
    }
    else {
      *puVar11 = 0;
      puVar11[1] = 0;
      puVar11[2] = 0;
      puVar11[3] = 0;
      uVar6 = puVar8[1];
      *puVar11 = *puVar8;
      puVar11[1] = uVar6;
      uVar6 = puVar8[3];
      puVar11[2] = puVar8[2];
      puVar11[3] = uVar6;
      puVar8[2] = 0;
      puVar8[3] = 0xf;
      *(undefined1 *)puVar8 = 0;
      param_1[1] = param_1[1] + 0x20;
    }
    FUN_180018de0(&local_110);
    FUN_180018de0(&local_130);
    uVar6 = FUN_180045890(&local_130);
    puVar8 = (undefined8 *)FUN_18001ffb0(&local_110,"SET_VELOCITY_LIMIT ACCEL_TO_DECEL=",uVar6);
    puVar11 = (undefined8 *)param_1[1];
    if (puVar11 == (undefined8 *)param_1[2]) {
      FUN_1800164a0(param_1,puVar11,puVar8);
    }
    else {
      *puVar11 = 0;
      puVar11[1] = 0;
      puVar11[2] = 0;
      puVar11[3] = 0;
      uVar6 = puVar8[1];
      *puVar11 = *puVar8;
      puVar11[1] = uVar6;
      uVar6 = puVar8[3];
      puVar11[2] = puVar8[2];
      puVar11[3] = uVar6;
      puVar8[2] = 0;
      puVar8[3] = 0xf;
      *(undefined1 *)puVar8 = 0;
      param_1[1] = param_1[1] + 0x20;
    }
    FUN_180018de0(&local_110);
    FUN_180018de0(&local_130);
  }
  cVar3 = FUN_180045ec0(param_2);
  if (cVar3 == '\0') {
    puVar11 = (undefined8 *)param_1[1];
    if (puVar11 == (undefined8 *)param_1[2]) {
      FUN_1800211a0(param_1,puVar11,"WOBBLE_STOP");
    }
    else {
      *puVar11 = 0;
      puVar11[1] = 0;
      puVar11[2] = 0;
      puVar11[3] = 0;
      FUN_1800155c0(puVar11,"WOBBLE_STOP",0xb);
      param_1[1] = param_1[1] + 0x20;
    }
  }
  else {
    local_130 = (undefined8 ******)0x0;
    ppppppuStack_128 = (undefined8 ******)0x0;
    local_120 = (undefined8 ******)0x0;
    ppppppuStack_118 = (undefined8 ******)0x0;
    FUN_1800155c0(&local_130,"wobble_work_speed",0x11);
    FUN_180023f50(param_2,&local_130);
    uVar6 = FUN_180045890(local_90);
    local_198[1] = (char *)0x3fb999999999999a;
    local_160 = 0xb;
    uStack_158 = 0xf;
    local_170 = (void *)0x735f656c62626f77;
    uStack_168 = 0x657a69;
    cVar3 = FUN_180055f90(param_2,&local_170);
    if (cVar3 == '\0') {
      if (*param_2 != 0) {
        FUN_1800240d0(*param_2,&local_170,local_198 + 1);
      }
    }
    else {
      FUN_180023f50(param_2,&local_170);
    }
    uVar10 = FUN_180045890(local_b0);
    uVar10 = FUN_18001ffb0(local_d0,"WOBBLE_START D=",uVar10);
    puVar11 = (undefined8 *)FUN_18001bfc0(uVar10,&DAT_18028eee4,3);
    local_110 = (void *)*puVar11;
    uStack_108 = puVar11[1];
    local_100 = puVar11[2];
    uStack_f8 = puVar11[3];
    puVar11[2] = 0;
    puVar11[3] = 0xf;
    *(undefined1 *)puVar11 = 0;
    local_1a4 = 0x59;
    FUN_180025820(&local_150,local_1a0,&local_110,uVar6);
    local_1a4 = 0xd9;
    puVar11 = (undefined8 *)param_1[1];
    if (puVar11 == (undefined8 *)param_1[2]) {
      FUN_1800164a0(param_1,puVar11,&local_150);
    }
    else {
      *puVar11 = local_150;
      puVar11[1] = uStack_148;
      puVar11[2] = local_140;
      puVar11[3] = uStack_138;
      local_140 = 0;
      uStack_138 = 0xf;
      local_150 = (char *)((ulonglong)local_150 & 0xffffffffffffff00);
      param_1[1] = param_1[1] + 0x20;
    }
    FUN_180018de0(&local_150);
    if (0xf < uStack_f8) {
      FUN_180019d40(&local_110,local_110);
    }
    FUN_180018de0(local_d0);
    FUN_180018de0(local_b0);
    if (0xf < uStack_158) {
      FUN_180019d40(&local_170,local_170);
    }
    local_160 = 0;
    uStack_158 = 0xf;
    local_170 = (void *)((ulonglong)local_170 & 0xffffffffffffff00);
    FUN_180018de0(local_90);
    if (0xf < ppppppuStack_118) {
      FUN_180019d40(&local_130,local_130);
    }
  }
  return param_1;
}



## raw_result.limitations

Ghidra Listing reports a terminal call at 0x1800366e6 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036832 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003697c to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036b51 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036d1c to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036d6c to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036fa3 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036fe9 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180037039 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x1800370f5 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x1800372d6 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180037315 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003736d to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003743a to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.

## normalized_result


/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Type propagation algorithm not settling */

undefined8 * FUN_1800365d0(undefined8 *param_1,longlong *param_2,char param_3)

{
  ulonglong uVar1;
  int iVar2;
  char cVar3;
  char cVar4;
  int iVar5;
  undefined8 uVar6;
  char **ppcVar7;
  undefined8 *puVar8;
  ulonglong *puVar9;
  undefined8 uVar10;
  undefined8 *puVar11;
  char **ppcVar12;
  undefined8 ******ppppppuVar13;
  void *pvVar14;
  undefined8 ******ppppppuVar15;
  longlong lVar16;
  undefined8 *******_Src;
  size_t _Size;
  undefined8 *******pppppppuVar17;
  ulonglong *_Src_00;
  ulonglong *_Size_00;
  bool bVar18;
  bool bVar19;
  char *pcVar20;
  undefined1 auStack_1d8 [32];
  char *local_1b8;
  undefined8 local_1b0;
  char local_1a8 [4];
  undefined4 local_1a4;
  char local_1a0;
  char *local_198 [2];
  char *local_188;
  char *local_180;
  undefined8 *local_178;
  void *local_170;
  undefined8 uStack_168;
  undefined8 local_160;
  ulonglong uStack_158;
  char *local_150;
  undefined8 uStack_148;
  undefined8 local_140;
  ulonglong uStack_138;
  undefined8 ******local_130;
  undefined8 ******ppppppuStack_128;
  undefined8 ******local_120;
  undefined8 ******ppppppuStack_118;
  void *local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  ulonglong uStack_f8;
  undefined8 *******local_f0;
  undefined8 uStack_e8;
  longlong local_e0;
  ulonglong uStack_d8;
  undefined1 local_d0 [32];
  undefined1 local_b0 [32];
  undefined1 local_90 [32];
  ulonglong local_70;
  
  local_70 = DAT_1803386c0 ^ (ulonglong)auStack_1d8;
  *param_1 = 0;
  param_1[1] = 0;
  param_1[2] = 0;
  local_1a4 = 1;
  lVar16 = -1;
  local_198[0] = (char *)CONCAT44(local_198[0]._4_4_,0xffffffff);
  local_160 = 8;
  uStack_158 = 0xf;
  local_170 = (void *)0x64695f726573616c;
  uStack_168 = 0;
  local_1a0 = param_3;
  local_178 = param_1;
  cVar3 = FUN_180055f90(param_2,&local_170);
  if (cVar3 == '\0') {
    if (*param_2 == 0) {
      iVar5 = (int)local_198[0];
    }
    else {
      iVar5 = FUN_180023c70(*param_2,&local_170,local_198);
    }
  }
  else {
    iVar5 = FUN_180023b10(param_2,&local_170);
  }
  if (0xf < uStack_158) {
    pvVar14 = local_170;
    if ((0xfff < uStack_158 + 1) &&
       (pvVar14 = *(void **)((longlong)local_170 + -8),
       0x1f < (ulonglong)((longlong)local_170 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pvVar14);
  }
  if (iVar5 == 1) {
    local_1a8[0] = '\0';
    local_160 = 0xc;
    uStack_158 = 0xf;
    local_170 = (void *)0x616e655f61706f6d;
    uStack_168 = 0x64656c62;
    cVar3 = FUN_180055f90(param_2,&local_170);
    if (cVar3 == '\0') {
      cVar3 = local_1a8[0];
      if (*param_2 != 0) {
        cVar3 = FUN_180025360(*param_2,&local_170,local_1a8);
      }
    }
    else {
      cVar3 = FUN_180025200(param_2,&local_170);
    }
    if (0xf < uStack_158) {
      FUN_180019d40(&local_170,local_170);
    }
    local_198[0] = (char *)((ulonglong)local_198[0] & 0xffffffff00000000);
    local_150 = (char *)0x0;
    uStack_148 = 0;
    local_140 = 0;
    uStack_138 = 0;
    FUN_1800155c0(&local_150,"mopa_pulse_width",0x10);
    cVar4 = FUN_180055f90(param_2,&local_150);
    if (cVar4 == '\0') {
      if (*param_2 == 0) {
        iVar5 = (int)local_198[0];
      }
      else {
        iVar5 = FUN_180023c70(*param_2,&local_150,local_198);
      }
    }
    else {
      iVar5 = FUN_180023b10(param_2,&local_150);
    }
    if (0xf < uStack_138) {
      pcVar20 = local_150;
      if ((0xfff < uStack_138 + 1) &&
         (pcVar20 = *(char **)(local_150 + -8), (char *)0x1f < local_150 + (-8 - (longlong)pcVar20))
         ) {
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(pcVar20);
    }
    if ((cVar3 != '\0') && (0 < iVar5)) {
      uStack_148 = 0;
      local_140 = 0;
      uStack_138 = 0xf;
      local_150 = (char *)0x0;
      local_1a4 = 0x201;
      FUN_180029cd0(&local_150,(double)iVar5);
      uVar6 = FUN_18001ffb0(&local_110,"MOPA_SET_PULSE_WIDTH DURATION=",&local_150);
      FUN_1800239a0(param_1,uVar6);
      FUN_180018de0(&local_110);
      local_1a4 = 1;
      if (0xf < uStack_138) {
        FUN_180019d40(&local_150,local_150);
      }
    }
    local_198[0] = (char *)((ulonglong)local_198[0] & 0xffffffff00000000);
    local_160 = 9;
    uStack_158 = 0xf;
    local_170 = (void *)0x6572665f61706f6d;
    uStack_168 = 0x71;
    cVar4 = FUN_180055f90(param_2,&local_170);
    if (cVar4 == '\0') {
      if (*param_2 == 0) {
        iVar5 = (int)local_198[0];
      }
      else {
        iVar5 = FUN_180023c70(*param_2,&local_170,local_198);
      }
    }
    else {
      iVar5 = FUN_180023b10(param_2,&local_170);
    }
    local_188 = (char *)CONCAT44(local_188._4_4_,iVar5);
    if (0xf < uStack_158) {
      pvVar14 = local_170;
      if ((0xfff < uStack_158 + 1) &&
         (pvVar14 = *(void **)((longlong)local_170 + -8),
         0x1f < (ulonglong)((longlong)local_170 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(pvVar14);
    }
    if (0 < iVar5) {
      ppcVar7 = &local_188;
      if (cVar3 == '\0') {
        if (iVar5 < 0x1e) {
          ppcVar7 = (char **)&DAT_18028ec40;
        }
        ppcVar12 = (char **)&DAT_18028ec44;
        bVar19 = SBORROW4(iVar5,0x3c);
        iVar2 = -0x3c;
        bVar18 = iVar5 == 0x3c;
      }
      else {
        ppcVar12 = (char **)&DAT_18028ec3c;
        bVar19 = SBORROW4(iVar5,4000);
        iVar2 = -4000;
        bVar18 = iVar5 == 4000;
      }
      if (!bVar18 && bVar19 == iVar5 + iVar2 < 0) {
        ppcVar7 = ppcVar12;
      }
      ppppppuStack_128 = (undefined8 ******)0x0;
      local_120 = (undefined8 ******)0x0;
      ppppppuStack_118 = (undefined8 ******)0xf;
      local_130 = (undefined8 ******)0x0;
      local_1a4 = 0x101;
      FUN_180029cd0(&local_130,(double)*(int *)ppcVar7);
      uVar6 = FUN_18001ffb0(&local_110,"MOPA_SET_FREQ FREQ=",&local_130);
      FUN_1800239a0(param_1,uVar6);
      FUN_180018de0(&local_110);
      local_1a4 = 1;
      FUN_180018de0(&local_130);
    }
  }
  local_1a8[0] = '\0';
  local_160 = 0xe;
  uStack_158 = 0xf;
  local_170 = (void *)0x5f776f6c66726961;
  uStack_168 = 0x656c7a7a6f6e;
  cVar3 = FUN_180055f90(param_2,&local_170);
  if (cVar3 == '\0') {
    cVar3 = local_1a8[0];
    if (*param_2 != 0) {
      cVar3 = FUN_180025360(*param_2,&local_170,local_1a8);
    }
  }
  else {
    cVar3 = FUN_180025200(param_2,&local_170);
  }
  local_198[1] = "AF_SWITCH ATTACHED=0";
  if (cVar3 != '\0') {
    local_198[1] = "AF_SWITCH ATTACHED=1";
  }
  puVar11 = (undefined8 *)param_1[1];
  if (puVar11 == (undefined8 *)param_1[2]) {
    FUN_180021670(param_1,puVar11,local_198 + 1);
  }
  else {
    *puVar11 = 0;
    puVar11[1] = 0;
    puVar11[2] = 0;
    puVar11[3] = 0;
    do {
      lVar16 = lVar16 + 1;
    } while (local_198[1][lVar16] != '\0');
    FUN_1800155c0(puVar11,local_198[1],lVar16);
    param_1[1] = param_1[1] + 0x20;
  }
  pppppppuVar17 = (undefined8 *******)0x0;
  if (0xf < uStack_158) {
    pvVar14 = local_170;
    if ((0xfff < uStack_158 + 1) &&
       (pvVar14 = *(void **)((longlong)local_170 + -8),
       0x1f < (ulonglong)((longlong)local_170 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pvVar14);
  }
  local_198[0] = (char *)0x4014000000000000;
  local_198[1] = (char *)0xc014000000000000;
  local_180 = (char *)0x0;
  local_160 = 0xd;
  uStack_158 = 0xf;
  local_170 = (void *)0x65645f726573616c;
  uStack_168 = 0x7375636f66;
  cVar3 = FUN_180055f90(param_2,&local_170);
  if (cVar3 == '\0') {
    pcVar20 = local_180;
    if (*param_2 != 0) {
      pcVar20 = (char *)FUN_1800240d0(*param_2,&local_170,&local_180);
    }
  }
  else {
    pcVar20 = (char *)FUN_180023f50(param_2,&local_170);
  }
  local_188 = pcVar20;
  ppcVar7 = local_198 + 1;
  if (-5.0 <= (double)pcVar20) {
    ppcVar7 = &local_188;
  }
  ppcVar12 = local_198;
  if ((double)pcVar20 <= 5.0) {
    ppcVar12 = ppcVar7;
  }
  uStack_148 = 0;
  local_140 = 0;
  uStack_138 = 0xf;
  local_150 = (char *)0x0;
  local_1a4 = 0x21;
  FUN_180029cd0(&local_150,*ppcVar12);
  puVar8 = (undefined8 *)FUN_18001ffb0(&local_130,"LASER_DEFOCUS DISTANCE=",&local_150);
  puVar11 = (undefined8 *)param_1[1];
  if (puVar11 == (undefined8 *)param_1[2]) {
    FUN_1800164a0(param_1,puVar11,puVar8);
  }
  else {
    *puVar11 = 0;
    puVar11[1] = 0;
    puVar11[2] = 0;
    puVar11[3] = 0;
    uVar6 = puVar8[1];
    *puVar11 = *puVar8;
    puVar11[1] = uVar6;
    uVar6 = puVar8[3];
    puVar11[2] = puVar8[2];
    puVar11[3] = uVar6;
    puVar8[2] = 0;
    puVar8[3] = 0xf;
    *(undefined1 *)puVar8 = 0;
    param_1[1] = param_1[1] + 0x20;
  }
  if (0xf < ppppppuStack_118) {
    FUN_180019d40(&local_130,local_130);
  }
  local_120 = (undefined8 ******)0x0;
  ppppppuStack_118 = (undefined8 ******)0xf;
  local_130 = (undefined8 ******)((ulonglong)local_130 & 0xffffffffffffff00);
  local_1a4 = 1;
  if (0xf < uStack_138) {
    pcVar20 = local_150;
    if ((0xfff < uStack_138 + 1) &&
       (pcVar20 = *(char **)(local_150 + -8), (char *)0x1f < local_150 + (-8 - (longlong)pcVar20)))
    {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pcVar20);
  }
  local_140 = 0;
  uStack_138 = 0xf;
  local_150 = (char *)((ulonglong)local_150 & 0xffffffffffffff00);
  if (0xf < uStack_158) {
    pvVar14 = local_170;
    if ((0xfff < uStack_158 + 1) &&
       (pvVar14 = *(void **)((longlong)local_170 + -8),
       0x1f < (ulonglong)((longlong)local_170 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pvVar14);
  }
  local_180 = (char *)CONCAT44(local_180._4_4_,100);
  local_188 = (char *)((ulonglong)local_188 & 0xffffffff00000000);
  local_198[1] = (char *)0x0;
  local_160 = 0xf;
  uStack_158 = 0xf;
  local_170 = (void *)0x6c665f726573616c;
  uStack_168 = 0x646c6569667461;
  cVar3 = FUN_180055f90(param_2,&local_170);
  if (cVar3 == '\0') {
    pcVar20 = local_198[1];
    if (*param_2 != 0) {
      pcVar20 = (char *)FUN_1800240d0(*param_2,&local_170,local_198 + 1);
    }
  }
  else {
    pcVar20 = (char *)FUN_180023f50(param_2,&local_170);
  }
  iVar5 = (int)(double)pcVar20;
  local_198[0] = (char *)CONCAT44(local_198[0]._4_4_,iVar5);
  ppcVar7 = &local_188;
  if (-1 < iVar5) {
    ppcVar7 = local_198;
  }
  ppcVar12 = &local_180;
  if (iVar5 < 0x65) {
    ppcVar12 = ppcVar7;
  }
  uStack_e8 = 0;
  local_e0 = 0;
  uStack_d8 = 0xf;
  local_f0 = (undefined8 *******)0x0;
  local_1a4 = 5;
  FUN_180029cd0(&local_f0,(double)*(int *)ppcVar12);
  if (uStack_d8 - local_e0 < 0x12) {
    local_1b0 = 0x12;
    local_1b8 = "LASER_FLATFIELD P=";
    pppppppuVar17 = (undefined8 *******)FUN_180022ac0(&local_f0,0x12,0,0);
  }
  else {
    _Src = &local_f0;
    if (0xf < uStack_d8) {
      _Src = local_f0;
    }
    if (((undefined8 *******)0x18028ee59 < _Src) ||
       ((char *)((longlong)_Src + local_e0) < "LASER_FLATFIELD P=")) {
      pppppppuVar17 = (undefined8 *******)0x12;
    }
    else if ("LASER_FLATFIELD P=" < _Src) {
      pppppppuVar17 = _Src + -0x30051dc9;
    }
    _Size = local_e0 + 1;
    local_e0 = local_e0 + 0x12;
    memmove((char *)((longlong)_Src + 0x12),_Src,_Size);
    memcpy(_Src,"LASER_FLATFIELD P=",(size_t)pppppppuVar17);
    memcpy((char *)((longlong)_Src + (longlong)pppppppuVar17),
           (char *)((longlong)pppppppuVar17 + 0x18028ee5a),0x12 - (longlong)pppppppuVar17);
    pppppppuVar17 = &local_f0;
  }
  local_130 = *pppppppuVar17;
  ppppppuStack_128 = pppppppuVar17[1];
  local_120 = pppppppuVar17[2];
  ppppppuStack_118 = pppppppuVar17[3];
  pppppppuVar17[2] = (undefined8 ******)0x0;
  pppppppuVar17[3] = (undefined8 ******)0xf;
  *(undefined1 *)pppppppuVar17 = 0;
  local_1a4 = 0x15;
  puVar9 = (ulonglong *)param_1[1];
  if (puVar9 == (ulonglong *)param_1[2]) {
    FUN_1800164a0(param_1,puVar9,&local_130);
    ppppppuVar15 = ppppppuStack_118;
  }
  else {
    *puVar9 = (ulonglong)local_130;
    puVar9[1] = (ulonglong)ppppppuStack_128;
    puVar9[2] = (ulonglong)local_120;
    puVar9[3] = (ulonglong)ppppppuStack_118;
    local_130 = (undefined8 ******)((ulonglong)local_130 & 0xffffffffffffff00);
    param_1[1] = param_1[1] + 0x20;
    ppppppuVar15 = (undefined8 ******)0xf;
  }
  if ((undefined8 ******)0xf < ppppppuVar15) {
    ppppppuVar13 = local_130;
    if ((0xfff < (longlong)ppppppuVar15 + 1U) &&
       (ppppppuVar13 = (undefined8 ******)local_130[-1],
       0x1f < (ulonglong)((longlong)local_130 + (-8 - (longlong)ppppppuVar13)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(ppppppuVar13);
  }
  local_1a4 = 0x11;
  if (0xf < uStack_d8) {
    pppppppuVar17 = local_f0;
    if ((0xfff < uStack_d8 + 1) &&
       (pppppppuVar17 = (undefined8 *******)local_f0[-1],
       (char *)0x1f < (char *)((longlong)local_f0 + (-8 - (longlong)pppppppuVar17)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pppppppuVar17);
  }
  local_e0 = 0;
  uStack_d8 = 0xf;
  local_f0 = (undefined8 *******)((ulonglong)local_f0 & 0xffffffffffffff00);
  if (0xf < uStack_158) {
    pvVar14 = local_170;
    if ((0xfff < uStack_158 + 1) &&
       (pvVar14 = *(void **)((longlong)local_170 + -8),
       0x1f < (ulonglong)((longlong)local_170 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pvVar14);
  }
  local_188 = (char *)CONCAT44(local_188._4_4_,100);
  local_150 = (char *)0x0;
  uStack_148 = 0;
  local_140 = 0;
  uStack_138 = 0;
  local_150 = (char *)FUN_18024833c(0x20);
  local_140 = 0x10;
  uStack_138 = 0x1f;
  builtin_strncpy(local_150,"air_assist_speed",0x11);
  cVar3 = FUN_180055f90(param_2,&local_150);
  if (cVar3 == '\0') {
    if (*param_2 == 0) {
      iVar5 = (int)local_188;
    }
    else {
      iVar5 = FUN_180023c70(*param_2,&local_150,&local_188);
    }
  }
  else {
    iVar5 = FUN_180023b10(param_2,&local_150);
  }
  if (0xf < uStack_138) {
    pcVar20 = local_150;
    if ((0xfff < uStack_138 + 1) &&
       (pcVar20 = *(char **)(local_150 + -8), (char *)0x1f < local_150 + (-8 - (longlong)pcVar20)))
    {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pcVar20);
  }
  local_1a8[0] = '\x01';
  local_160 = 10;
  uStack_158 = 0xf;
  local_170 = (void *)0x697373615f726961;
  uStack_168 = 0x7473;
  local_1a4 = 0x13;
  cVar3 = FUN_180055f90(param_2,&local_170);
  if (cVar3 == '\0') {
    cVar3 = local_1a8[0];
    if (*param_2 != 0) {
      cVar3 = FUN_180025360(*param_2,&local_170,local_1a8);
    }
  }
  else {
    cVar3 = FUN_180025200(param_2,&local_170);
  }
  if ((cVar3 == '\0') || (iVar5 < 1)) {
    iVar5 = 0;
  }
  puVar9 = (ulonglong *)FUN_1800457c0(&local_110,iVar5);
  uVar1 = puVar9[2];
  if (puVar9[3] - uVar1 < 0x15) {
    local_1b0 = 0x15;
    local_1b8 = "AIR_ASSIST_SET SPEED=";
    puVar9 = (ulonglong *)FUN_180022ac0(puVar9,0x15,0,0);
  }
  else {
    puVar9[2] = uVar1 + 0x15;
    _Src_00 = puVar9;
    if (0xf < puVar9[3]) {
      _Src_00 = (ulonglong *)*puVar9;
    }
    if (((ulonglong *)0x18028ee9c < _Src_00) ||
       ((char *)((longlong)_Src_00 + uVar1) < "AIR_ASSIST_SET SPEED=")) {
      _Size_00 = (ulonglong *)0x15;
    }
    else if ("AIR_ASSIST_SET SPEED=" < _Src_00) {
      _Size_00 = _Src_00 + -0x30051dd1;
    }
    else {
      _Size_00 = (ulonglong *)0x0;
    }
    memmove((undefined1 *)((longlong)_Src_00 + 0x15),_Src_00,uVar1 + 1);
    memcpy(_Src_00,"AIR_ASSIST_SET SPEED=",(size_t)_Size_00);
    memcpy((undefined1 *)((longlong)_Src_00 + (longlong)_Size_00),
           (undefined1 *)((longlong)_Size_00 + 0x18028ee9d),0x15 - (longlong)_Size_00);
  }
  local_130 = (undefined8 ******)*puVar9;
  ppppppuStack_128 = (undefined8 ******)puVar9[1];
  local_120 = (undefined8 ******)puVar9[2];
  ppppppuStack_118 = (undefined8 ******)puVar9[3];
  puVar9[2] = 0;
  puVar9[3] = 0xf;
  *(undefined1 *)puVar9 = 0;
  local_1a4 = 0x1b;
  puVar9 = (ulonglong *)param_1[1];
  if (puVar9 == (ulonglong *)param_1[2]) {
    FUN_1800164a0(param_1,puVar9,&local_130);
    ppppppuVar15 = ppppppuStack_118;
  }
  else {
    *puVar9 = (ulonglong)local_130;
    puVar9[1] = (ulonglong)ppppppuStack_128;
    puVar9[2] = (ulonglong)local_120;
    puVar9[3] = (ulonglong)ppppppuStack_118;
    local_130 = (undefined8 ******)((ulonglong)local_130 & 0xffffffffffffff00);
    param_1[1] = param_1[1] + 0x20;
    ppppppuVar15 = (undefined8 ******)0xf;
  }
  if (0xf < ppppppuVar15) {
    ppppppuVar13 = local_130;
    if ((0xfff < (longlong)ppppppuVar15 + 1U) &&
       (ppppppuVar13 = (undefined8 ******)local_130[-1],
       0x1f < (ulonglong)((longlong)local_130 + (-8 - (longlong)ppppppuVar13)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(ppppppuVar13);
  }
  if (0xf < uStack_f8) {
    pvVar14 = local_110;
    if ((0xfff < uStack_f8 + 1) &&
       (pvVar14 = *(void **)((longlong)local_110 + -8),
       0x1f < (ulonglong)((longlong)local_110 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pvVar14);
  }
  local_100 = 0;
  uStack_f8 = 0xf;
  local_110 = (void *)((ulonglong)local_110 & 0xffffffffffffff00);
  local_1a4 = 0x19;
  if (0xf < uStack_158) {
    pvVar14 = local_170;
    if ((0xfff < uStack_158 + 1) &&
       (pvVar14 = *(void **)((longlong)local_170 + -8),
       0x1f < (ulonglong)((longlong)local_170 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pvVar14);
  }
  if (local_1a0 != '\0') {
    local_198[1] = (char *)0x408f400000000000;
    uStack_168 = 0;
    local_160 = 5;
    uStack_158 = 0xf;
    local_170 = (void *)0x6c65636361;
    cVar3 = FUN_180055f90(param_2,&local_170);
    if (cVar3 == '\0') {
      if (*param_2 != 0) {
        FUN_1800240d0(*param_2,&local_170,local_198 + 1);
      }
    }
    else {
      FUN_180023f50(param_2,&local_170);
    }
    if (0xf < uStack_158) {
      pvVar14 = local_170;
      if ((0xfff < uStack_158 + 1) &&
         (pvVar14 = *(void **)((longlong)local_170 + -8),
         0x1f < (ulonglong)((longlong)local_170 + (-8 - (longlong)pvVar14)))) {
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(pvVar14);
    }
    uVar6 = FUN_180045890(&local_130);
    puVar8 = (undefined8 *)FUN_18001ffb0(&local_110,"SET_VELOCITY_LIMIT ACCEL=",uVar6);
    puVar11 = (undefined8 *)param_1[1];
    if (puVar11 == (undefined8 *)param_1[2]) {
      FUN_1800164a0(param_1,puVar11,puVar8);
    }
    else {
      *puVar11 = 0;
      puVar11[1] = 0;
      puVar11[2] = 0;
      puVar11[3] = 0;
      uVar6 = puVar8[1];
      *puVar11 = *puVar8;
      puVar11[1] = uVar6;
      uVar6 = puVar8[3];
      puVar11[2] = puVar8[2];
      puVar11[3] = uVar6;
      puVar8[2] = 0;
      puVar8[3] = 0xf;
      *(undefined1 *)puVar8 = 0;
      param_1[1] = param_1[1] + 0x20;
    }
    FUN_180018de0(&local_110);
    FUN_180018de0(&local_130);
    uVar6 = FUN_180045890(&local_130);
    puVar8 = (undefined8 *)FUN_18001ffb0(&local_110,"SET_VELOCITY_LIMIT ACCEL_TO_DECEL=",uVar6);
    puVar11 = (undefined8 *)param_1[1];
    if (puVar11 == (undefined8 *)param_1[2]) {
      FUN_1800164a0(param_1,puVar11,puVar8);
    }
    else {
      *puVar11 = 0;
      puVar11[1] = 0;
      puVar11[2] = 0;
      puVar11[3] = 0;
      uVar6 = puVar8[1];
      *puVar11 = *puVar8;
      puVar11[1] = uVar6;
      uVar6 = puVar8[3];
      puVar11[2] = puVar8[2];
      puVar11[3] = uVar6;
      puVar8[2] = 0;
      puVar8[3] = 0xf;
      *(undefined1 *)puVar8 = 0;
      param_1[1] = param_1[1] + 0x20;
    }
    FUN_180018de0(&local_110);
    FUN_180018de0(&local_130);
  }
  cVar3 = FUN_180045ec0(param_2);
  if (cVar3 == '\0') {
    puVar11 = (undefined8 *)param_1[1];
    if (puVar11 == (undefined8 *)param_1[2]) {
      FUN_1800211a0(param_1,puVar11,"WOBBLE_STOP");
    }
    else {
      *puVar11 = 0;
      puVar11[1] = 0;
      puVar11[2] = 0;
      puVar11[3] = 0;
      FUN_1800155c0(puVar11,"WOBBLE_STOP",0xb);
      param_1[1] = param_1[1] + 0x20;
    }
  }
  else {
    local_130 = (undefined8 ******)0x0;
    ppppppuStack_128 = (undefined8 ******)0x0;
    local_120 = (undefined8 ******)0x0;
    ppppppuStack_118 = (undefined8 ******)0x0;
    FUN_1800155c0(&local_130,"wobble_work_speed",0x11);
    FUN_180023f50(param_2,&local_130);
    uVar6 = FUN_180045890(local_90);
    local_198[1] = (char *)0x3fb999999999999a;
    local_160 = 0xb;
    uStack_158 = 0xf;
    local_170 = (void *)0x735f656c62626f77;
    uStack_168 = 0x657a69;
    cVar3 = FUN_180055f90(param_2,&local_170);
    if (cVar3 == '\0') {
      if (*param_2 != 0) {
        FUN_1800240d0(*param_2,&local_170,local_198 + 1);
      }
    }
    else {
      FUN_180023f50(param_2,&local_170);
    }
    uVar10 = FUN_180045890(local_b0);
    uVar10 = FUN_18001ffb0(local_d0,"WOBBLE_START D=",uVar10);
    puVar11 = (undefined8 *)FUN_18001bfc0(uVar10,&DAT_18028eee4,3);
    local_110 = (void *)*puVar11;
    uStack_108 = puVar11[1];
    local_100 = puVar11[2];
    uStack_f8 = puVar11[3];
    puVar11[2] = 0;
    puVar11[3] = 0xf;
    *(undefined1 *)puVar11 = 0;
    local_1a4 = 0x59;
    FUN_180025820(&local_150,local_1a0,&local_110,uVar6);
    local_1a4 = 0xd9;
    puVar11 = (undefined8 *)param_1[1];
    if (puVar11 == (undefined8 *)param_1[2]) {
      FUN_1800164a0(param_1,puVar11,&local_150);
    }
    else {
      *puVar11 = local_150;
      puVar11[1] = uStack_148;
      puVar11[2] = local_140;
      puVar11[3] = uStack_138;
      local_140 = 0;
      uStack_138 = 0xf;
      local_150 = (char *)((ulonglong)local_150 & 0xffffffffffffff00);
      param_1[1] = param_1[1] + 0x20;
    }
    FUN_180018de0(&local_150);
    if (0xf < uStack_f8) {
      FUN_180019d40(&local_110,local_110);
    }
    FUN_180018de0(local_d0);
    FUN_180018de0(local_b0);
    if (0xf < uStack_158) {
      FUN_180019d40(&local_170,local_170);
    }
    local_160 = 0;
    uStack_158 = 0xf;
    local_170 = (void *)((ulonglong)local_170 & 0xffffffffffffff00);
    FUN_180018de0(local_90);
    if (0xf < ppppppuStack_118) {
      FUN_180019d40(&local_130,local_130);
    }
  }
  return param_1;
}



## confidence

observed

## authority

shipped-artifact

## environment

null

## limitations

The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.,The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.,Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.,Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.,Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.,External functions and functions without an analyzable body return null; other decompiler failures remain explicit.,Ghidra Listing reports a terminal call at 0x1800366e6 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036832 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003697c to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036b51 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036d1c to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036d6c to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036fa3 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036fe9 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180037039 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x1800370f5 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x1800372d6 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180037315 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003736d to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003743a to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Windows Ghidra P0 accepts native x86, x86-64, ARMNT (Thumb-2) and ARM64 PE applications and DLLs, and explicitly selected raw 6502 images; managed, hostile, sensitive, and mutable-path targets are unsupported.,The Windows bridge uses authenticated IPv4 loopback because Node path-based IPC does not expose Java AF_UNIX sockets; the endpoint file contains no bearer token.,Windows sessions require the matching packaged Windows x64 native addon, local NTFS targets and runtimes, and Windows 10 or later. Native handles enforce path admission, private DACLs, and Job Object ownership automatically.

## relationship_coverage

null

## locations



## evidence_links

