## evidence_id

ev_663d78043919dabb71a09a27c1972cdd523ec739a1e2119bcd604957f4fdeecb

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
| procedure | 0x180060f80 |

## raw_result.value


/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Type propagation algorithm not settling */

undefined8 *
FUN_180060a30(undefined8 *param_1,undefined8 param_2,longlong param_3,int param_4,longlong param_5)

{
  ulonglong uVar1;
  void *pvVar2;
  int iVar3;
  longlong *plVar4;
  undefined8 ******ppppppuVar5;
  undefined1 *puVar6;
  ulonglong uVar7;
  longlong lVar8;
  undefined8 uVar9;
  basic_ostream<char,std::char_traits<char>_> *this;
  int *piVar10;
  undefined8 *******pppppppuVar11;
  undefined8 *puVar12;
  char cVar13;
  ulonglong uVar14;
  undefined8 *puVar15;
  undefined8 *puVar16;
  void *pvVar17;
  int iVar18;
  int iVar19;
  undefined1 auStack_1d8 [32];
  char local_1b8;
  int local_1b4;
  int local_1b0;
  undefined4 local_1ac;
  int local_1a8;
  longlong local_1a0;
  longlong local_198;
  undefined8 local_190;
  undefined8 *local_188;
  undefined1 local_180 [48];
  undefined8 *******local_150;
  undefined8 uStack_148;
  ulonglong local_140;
  ulonglong local_138;
  void *local_128;
  undefined8 uStack_120;
  ulonglong local_118;
  ulonglong local_110;
  undefined8 local_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined1 local_c8;
  undefined7 uStack_c7;
  undefined8 local_b8;
  ulonglong local_b0;
  undefined1 local_a8;
  undefined7 uStack_a7;
  undefined8 local_98;
  ulonglong local_90;
  int local_88;
  int local_84;
  undefined4 local_80;
  undefined4 local_7c;
  undefined4 local_78;
  undefined4 local_74;
  undefined4 local_70;
  undefined1 local_6c;
  undefined4 local_68;
  ulonglong local_58;
  
  local_58 = DAT_1803386c0 ^ (ulonglong)auStack_1d8;
  local_1a8 = param_4;
  local_198 = param_3;
  local_190 = param_2;
  local_188 = param_1;
  local_1a0 = param_5;
  local_1ac = 0;
  local_1b0 = 0;
  piVar10 = &local_1b0;
  if (-1 < *(int *)(param_5 + 0x40)) {
    piVar10 = (int *)(param_5 + 0x40);
  }
  iVar18 = *(int *)(param_5 + 0x44);
  if (iVar18 < 0) {
    iVar18 = *(int *)(param_3 + 0x78);
  }
  if (*(int *)(param_3 + 0x78) < iVar18) {
    iVar18 = *(int *)(param_3 + 0x78);
  }
  local_1b4 = *piVar10;
  if (iVar18 < *piVar10) {
    local_1b4 = iVar18;
  }
  *param_1 = 0;
  param_1[1] = 0;
  param_1[2] = 0;
  local_1ac = 1;
  cVar13 = '\x01';
  local_1b8 = '\x01';
  iVar3 = local_1b4;
  do {
    if (iVar18 <= iVar3) {
      puVar16 = (undefined8 *)param_1[1];
      for (puVar15 = (undefined8 *)*param_1; puVar15 != puVar16; puVar15 = puVar15 + 0xb) {
        uVar9 = FUN_1800152f0(cout_exref,"binary_gcode_file: ");
        puVar12 = puVar15;
        if (0xf < (ulonglong)puVar15[3]) {
          puVar12 = (undefined8 *)*puVar15;
        }
        this = (basic_ostream<char,std::char_traits<char>_> *)
               FUN_180016720(uVar9,puVar12,puVar15[2]);
        std::basic_ostream<char,std::char_traits<char>_>::operator<<(this,FUN_1800178f0);
      }
      return param_1;
    }
    iVar19 = iVar18;
    if ((0 < local_1a8) && (iVar19 = local_1a8 + iVar3, iVar18 < local_1a8 + iVar3)) {
      iVar19 = iVar18;
    }
    plVar4 = (longlong *)FUN_1800601d0(local_190);
    lVar8 = *plVar4;
    puVar16 = (undefined8 *)(lVar8 + 0x28);
    local_150 = (undefined8 *******)0x0;
    uStack_148 = 0;
    local_140 = 0;
    local_138 = 0;
    uVar1 = *(ulonglong *)(lVar8 + 0x38);
    if (0xf < *(ulonglong *)(lVar8 + 0x40)) {
      puVar16 = (undefined8 *)*puVar16;
    }
    if (0x7fffffffffffffff < uVar1) {
                    /* WARNING: Subroutine does not return */
      FUN_18001ae90();
    }
    if (uVar1 < 0x10) {
      local_138 = 0xf;
      local_150 = (undefined8 *******)*puVar16;
      uStack_148 = puVar16[1];
      local_140 = uVar1;
    }
    else {
      uVar14 = uVar1 | 0xf;
      if (uVar14 < 0x8000000000000000) {
        if (uVar14 < 0x16) {
          uVar14 = 0x16;
        }
        if (0xfff < uVar14 + 1) {
          uVar7 = uVar14 + 0x28;
          if (uVar7 <= uVar14 + 1) {
                    /* WARNING: Subroutine does not return */
            FUN_18001a930();
          }
          goto LAB_180060b9c;
        }
        if (uVar14 == 0xffffffffffffffff) {
          local_150 = (undefined8 *******)0x0;
        }
        else {
          local_150 = (undefined8 *******)FUN_18024833c();
        }
      }
      else {
        uVar14 = 0x7fffffffffffffff;
        uVar7 = 0x8000000000000027;
LAB_180060b9c:
        ppppppuVar5 = (undefined8 ******)FUN_18024833c(uVar7);
        if (ppppppuVar5 == (undefined8 ******)0x0) goto LAB_180060f65;
        local_150 = (undefined8 *******)((longlong)ppppppuVar5 + 0x27U & 0xffffffffffffffe0);
        local_150[-1] = ppppppuVar5;
      }
      local_140 = uVar1;
      local_138 = uVar14;
      memcpy(local_150,puVar16,uVar1 + 1);
      cVar13 = local_1b8;
    }
    (**(code **)(*(longlong *)*plVar4 + 0x10))();
    lVar8 = local_1a0;
    FUN_180017eb0(&local_c8,local_1a0);
    FUN_180017eb0(&local_a8);
    uVar1 = local_140;
    local_80 = *(undefined4 *)(lVar8 + 0x48);
    local_7c = *(undefined4 *)(lVar8 + 0x4c);
    local_78 = *(undefined4 *)(lVar8 + 0x50);
    local_74 = *(undefined4 *)(lVar8 + 0x54);
    local_70 = *(undefined4 *)(lVar8 + 0x58);
    local_6c = *(undefined1 *)(lVar8 + 0x5c);
    local_68 = *(undefined4 *)(lVar8 + 0x60);
    local_88 = local_1b4;
    pvVar17 = (void *)0x0;
    local_84 = iVar19;
    if (cVar13 == '\0') {
      local_b8 = 0;
      puVar6 = &local_c8;
      if (0xf < local_b0) {
        puVar6 = (undefined1 *)CONCAT71(uStack_c7,local_c8);
      }
      *puVar6 = 0;
    }
    if (iVar19 != iVar18) {
      local_98 = 0;
      puVar6 = &local_a8;
      if (0xf < local_90) {
        puVar6 = (undefined1 *)CONCAT71(uStack_a7,local_a8);
      }
      *puVar6 = 0;
    }
    uStack_120 = 0;
    local_118 = 0;
    local_110 = 0xf;
    local_128 = (void *)0x0;
    local_108 = 0;
    local_100 = 0;
    uStack_f8 = 0;
    local_f0 = 0;
    uStack_e8 = 0;
    local_e0 = 0;
    uStack_d8 = 0;
    pppppppuVar11 = &local_150;
    if (0xf < local_138) {
      pppppppuVar11 = local_150;
    }
    if (local_140 < 0x10) {
      local_118 = local_140;
      memmove(&local_128,pppppppuVar11,local_140);
      *(undefined1 *)((longlong)&local_128 + uVar1) = 0;
    }
    else {
      if (0x7fffffffffffffff < local_140) {
                    /* WARNING: Subroutine does not return */
        FUN_18001ae90();
      }
      uVar14 = local_140 | 0xf;
      if (uVar14 < 0x8000000000000000) {
        if (uVar14 < 0x16) {
          uVar14 = 0x16;
        }
        if (0xfff < uVar14 + 1) {
          uVar7 = uVar14 + 0x28;
          if (uVar7 <= uVar14 + 1) {
                    /* WARNING: Subroutine does not return */
            FUN_18001a930();
          }
          goto LAB_180060d4f;
        }
        if (uVar14 != 0xffffffffffffffff) {
          pvVar17 = (void *)FUN_18024833c();
        }
      }
      else {
        uVar14 = 0x7fffffffffffffff;
        uVar7 = 0x8000000000000027;
LAB_180060d4f:
        lVar8 = FUN_18024833c(uVar7);
        if (lVar8 == 0) {
                    /* WARNING: Subroutine does not return */
          _invalid_parameter_noinfo_noreturn();
        }
        pvVar17 = (void *)(lVar8 + 0x27U & 0xffffffffffffffe0);
        *(longlong *)((longlong)pvVar17 - 8) = lVar8;
      }
      local_118 = uVar1;
      local_110 = uVar14;
      memcpy(pvVar17,pppppppuVar11,uVar1);
      *(undefined1 *)((longlong)pvVar17 + uVar1) = 0;
      local_128 = pvVar17;
    }
    local_108 = CONCAT44(iVar19,local_1b4);
    puVar16 = (undefined8 *)FUN_18009af60(local_180,&local_150,local_198,&local_c8);
    local_100 = *puVar16;
    uStack_f8 = puVar16[1];
    local_f0 = puVar16[2];
    uStack_e8 = puVar16[3];
    local_e0 = puVar16[4];
    uStack_d8 = puVar16[5];
    lVar8 = param_1[1];
    if (lVar8 == param_1[2]) {
      FUN_18005fdc0(param_1,lVar8,&local_128);
    }
    else {
      FUN_180017eb0(lVar8,&local_128);
      *(undefined4 *)(lVar8 + 0x20) = (undefined4)local_108;
      *(undefined4 *)(lVar8 + 0x24) = local_108._4_4_;
      *(undefined8 *)(lVar8 + 0x28) = local_100;
      *(undefined8 *)(lVar8 + 0x30) = uStack_f8;
      *(undefined8 *)(lVar8 + 0x38) = local_f0;
      *(undefined8 *)(lVar8 + 0x40) = uStack_e8;
      *(undefined8 *)(lVar8 + 0x48) = local_e0;
      *(undefined8 *)(lVar8 + 0x50) = uStack_d8;
      param_1[1] = param_1[1] + 0x58;
    }
    cVar13 = '\0';
    local_1b8 = '\0';
    local_1b4 = iVar19;
    if (0xf < local_110) {
      pvVar17 = local_128;
      if ((0xfff < local_110 + 1) &&
         (pvVar17 = *(void **)((longlong)local_128 + -8),
         0x1f < (ulonglong)((longlong)local_128 + (-8 - (longlong)pvVar17)))) {
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(pvVar17);
    }
    if (0xf < local_90) {
      pvVar2 = (void *)CONCAT71(uStack_a7,local_a8);
      pvVar17 = pvVar2;
      if ((0xfff < local_90 + 1) &&
         (pvVar17 = *(void **)((longlong)pvVar2 + -8),
         0x1f < (ulonglong)((longlong)pvVar2 + (-8 - (longlong)pvVar17)))) goto LAB_180060f5e;
      free(pvVar17);
    }
    local_98 = 0;
    local_90 = 0xf;
    local_a8 = 0;
    if (0xf < local_b0) {
      pvVar2 = (void *)CONCAT71(uStack_c7,local_c8);
      pvVar17 = pvVar2;
      if ((0xfff < local_b0 + 1) &&
         (pvVar17 = *(void **)((longlong)pvVar2 + -8),
         0x1f < (ulonglong)((longlong)pvVar2 + (-8 - (longlong)pvVar17)))) {
LAB_180060f5e:
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(pvVar17);
    }
    local_b8 = 0;
    local_b0 = 0xf;
    local_c8 = 0;
    iVar3 = iVar19;
    if (0xf < local_138) {
      pppppppuVar11 = local_150;
      if ((0xfff < local_138 + 1) &&
         (pppppppuVar11 = (undefined8 *******)local_150[-1],
         0x1f < (ulonglong)((longlong)local_150 + (-8 - (longlong)pppppppuVar11)))) {
LAB_180060f65:
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(pppppppuVar11);
    }
  } while( true );
}



## raw_result.limitations

Ghidra Listing reports a terminal call at 0x180060f50 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180060f57 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180060f5e to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180060f65 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180060feb to 0x18001ae90 FUN_18001ae90 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180060ff1 to 0x18001a930 FUN_18001a930 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180060ff7 to 0x18001a930 FUN_18001a930 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180060ffd to 0x18001ae90 FUN_18001ae90 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.

## normalized_result


/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Type propagation algorithm not settling */

undefined8 *
FUN_180060a30(undefined8 *param_1,undefined8 param_2,longlong param_3,int param_4,longlong param_5)

{
  ulonglong uVar1;
  void *pvVar2;
  int iVar3;
  longlong *plVar4;
  undefined8 ******ppppppuVar5;
  undefined1 *puVar6;
  ulonglong uVar7;
  longlong lVar8;
  undefined8 uVar9;
  basic_ostream<char,std::char_traits<char>_> *this;
  int *piVar10;
  undefined8 *******pppppppuVar11;
  undefined8 *puVar12;
  char cVar13;
  ulonglong uVar14;
  undefined8 *puVar15;
  undefined8 *puVar16;
  void *pvVar17;
  int iVar18;
  int iVar19;
  undefined1 auStack_1d8 [32];
  char local_1b8;
  int local_1b4;
  int local_1b0;
  undefined4 local_1ac;
  int local_1a8;
  longlong local_1a0;
  longlong local_198;
  undefined8 local_190;
  undefined8 *local_188;
  undefined1 local_180 [48];
  undefined8 *******local_150;
  undefined8 uStack_148;
  ulonglong local_140;
  ulonglong local_138;
  void *local_128;
  undefined8 uStack_120;
  ulonglong local_118;
  ulonglong local_110;
  undefined8 local_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined1 local_c8;
  undefined7 uStack_c7;
  undefined8 local_b8;
  ulonglong local_b0;
  undefined1 local_a8;
  undefined7 uStack_a7;
  undefined8 local_98;
  ulonglong local_90;
  int local_88;
  int local_84;
  undefined4 local_80;
  undefined4 local_7c;
  undefined4 local_78;
  undefined4 local_74;
  undefined4 local_70;
  undefined1 local_6c;
  undefined4 local_68;
  ulonglong local_58;
  
  local_58 = DAT_1803386c0 ^ (ulonglong)auStack_1d8;
  local_1a8 = param_4;
  local_198 = param_3;
  local_190 = param_2;
  local_188 = param_1;
  local_1a0 = param_5;
  local_1ac = 0;
  local_1b0 = 0;
  piVar10 = &local_1b0;
  if (-1 < *(int *)(param_5 + 0x40)) {
    piVar10 = (int *)(param_5 + 0x40);
  }
  iVar18 = *(int *)(param_5 + 0x44);
  if (iVar18 < 0) {
    iVar18 = *(int *)(param_3 + 0x78);
  }
  if (*(int *)(param_3 + 0x78) < iVar18) {
    iVar18 = *(int *)(param_3 + 0x78);
  }
  local_1b4 = *piVar10;
  if (iVar18 < *piVar10) {
    local_1b4 = iVar18;
  }
  *param_1 = 0;
  param_1[1] = 0;
  param_1[2] = 0;
  local_1ac = 1;
  cVar13 = '\x01';
  local_1b8 = '\x01';
  iVar3 = local_1b4;
  do {
    if (iVar18 <= iVar3) {
      puVar16 = (undefined8 *)param_1[1];
      for (puVar15 = (undefined8 *)*param_1; puVar15 != puVar16; puVar15 = puVar15 + 0xb) {
        uVar9 = FUN_1800152f0(cout_exref,"binary_gcode_file: ");
        puVar12 = puVar15;
        if (0xf < (ulonglong)puVar15[3]) {
          puVar12 = (undefined8 *)*puVar15;
        }
        this = (basic_ostream<char,std::char_traits<char>_> *)
               FUN_180016720(uVar9,puVar12,puVar15[2]);
        std::basic_ostream<char,std::char_traits<char>_>::operator<<(this,FUN_1800178f0);
      }
      return param_1;
    }
    iVar19 = iVar18;
    if ((0 < local_1a8) && (iVar19 = local_1a8 + iVar3, iVar18 < local_1a8 + iVar3)) {
      iVar19 = iVar18;
    }
    plVar4 = (longlong *)FUN_1800601d0(local_190);
    lVar8 = *plVar4;
    puVar16 = (undefined8 *)(lVar8 + 0x28);
    local_150 = (undefined8 *******)0x0;
    uStack_148 = 0;
    local_140 = 0;
    local_138 = 0;
    uVar1 = *(ulonglong *)(lVar8 + 0x38);
    if (0xf < *(ulonglong *)(lVar8 + 0x40)) {
      puVar16 = (undefined8 *)*puVar16;
    }
    if (0x7fffffffffffffff < uVar1) {
                    /* WARNING: Subroutine does not return */
      FUN_18001ae90();
    }
    if (uVar1 < 0x10) {
      local_138 = 0xf;
      local_150 = (undefined8 *******)*puVar16;
      uStack_148 = puVar16[1];
      local_140 = uVar1;
    }
    else {
      uVar14 = uVar1 | 0xf;
      if (uVar14 < 0x8000000000000000) {
        if (uVar14 < 0x16) {
          uVar14 = 0x16;
        }
        if (0xfff < uVar14 + 1) {
          uVar7 = uVar14 + 0x28;
          if (uVar7 <= uVar14 + 1) {
                    /* WARNING: Subroutine does not return */
            FUN_18001a930();
          }
          goto LAB_180060b9c;
        }
        if (uVar14 == 0xffffffffffffffff) {
          local_150 = (undefined8 *******)0x0;
        }
        else {
          local_150 = (undefined8 *******)FUN_18024833c();
        }
      }
      else {
        uVar14 = 0x7fffffffffffffff;
        uVar7 = 0x8000000000000027;
LAB_180060b9c:
        ppppppuVar5 = (undefined8 ******)FUN_18024833c(uVar7);
        if (ppppppuVar5 == (undefined8 ******)0x0) goto LAB_180060f65;
        local_150 = (undefined8 *******)((longlong)ppppppuVar5 + 0x27U & 0xffffffffffffffe0);
        local_150[-1] = ppppppuVar5;
      }
      local_140 = uVar1;
      local_138 = uVar14;
      memcpy(local_150,puVar16,uVar1 + 1);
      cVar13 = local_1b8;
    }
    (**(code **)(*(longlong *)*plVar4 + 0x10))();
    lVar8 = local_1a0;
    FUN_180017eb0(&local_c8,local_1a0);
    FUN_180017eb0(&local_a8);
    uVar1 = local_140;
    local_80 = *(undefined4 *)(lVar8 + 0x48);
    local_7c = *(undefined4 *)(lVar8 + 0x4c);
    local_78 = *(undefined4 *)(lVar8 + 0x50);
    local_74 = *(undefined4 *)(lVar8 + 0x54);
    local_70 = *(undefined4 *)(lVar8 + 0x58);
    local_6c = *(undefined1 *)(lVar8 + 0x5c);
    local_68 = *(undefined4 *)(lVar8 + 0x60);
    local_88 = local_1b4;
    pvVar17 = (void *)0x0;
    local_84 = iVar19;
    if (cVar13 == '\0') {
      local_b8 = 0;
      puVar6 = &local_c8;
      if (0xf < local_b0) {
        puVar6 = (undefined1 *)CONCAT71(uStack_c7,local_c8);
      }
      *puVar6 = 0;
    }
    if (iVar19 != iVar18) {
      local_98 = 0;
      puVar6 = &local_a8;
      if (0xf < local_90) {
        puVar6 = (undefined1 *)CONCAT71(uStack_a7,local_a8);
      }
      *puVar6 = 0;
    }
    uStack_120 = 0;
    local_118 = 0;
    local_110 = 0xf;
    local_128 = (void *)0x0;
    local_108 = 0;
    local_100 = 0;
    uStack_f8 = 0;
    local_f0 = 0;
    uStack_e8 = 0;
    local_e0 = 0;
    uStack_d8 = 0;
    pppppppuVar11 = &local_150;
    if (0xf < local_138) {
      pppppppuVar11 = local_150;
    }
    if (local_140 < 0x10) {
      local_118 = local_140;
      memmove(&local_128,pppppppuVar11,local_140);
      *(undefined1 *)((longlong)&local_128 + uVar1) = 0;
    }
    else {
      if (0x7fffffffffffffff < local_140) {
                    /* WARNING: Subroutine does not return */
        FUN_18001ae90();
      }
      uVar14 = local_140 | 0xf;
      if (uVar14 < 0x8000000000000000) {
        if (uVar14 < 0x16) {
          uVar14 = 0x16;
        }
        if (0xfff < uVar14 + 1) {
          uVar7 = uVar14 + 0x28;
          if (uVar7 <= uVar14 + 1) {
                    /* WARNING: Subroutine does not return */
            FUN_18001a930();
          }
          goto LAB_180060d4f;
        }
        if (uVar14 != 0xffffffffffffffff) {
          pvVar17 = (void *)FUN_18024833c();
        }
      }
      else {
        uVar14 = 0x7fffffffffffffff;
        uVar7 = 0x8000000000000027;
LAB_180060d4f:
        lVar8 = FUN_18024833c(uVar7);
        if (lVar8 == 0) {
                    /* WARNING: Subroutine does not return */
          _invalid_parameter_noinfo_noreturn();
        }
        pvVar17 = (void *)(lVar8 + 0x27U & 0xffffffffffffffe0);
        *(longlong *)((longlong)pvVar17 - 8) = lVar8;
      }
      local_118 = uVar1;
      local_110 = uVar14;
      memcpy(pvVar17,pppppppuVar11,uVar1);
      *(undefined1 *)((longlong)pvVar17 + uVar1) = 0;
      local_128 = pvVar17;
    }
    local_108 = CONCAT44(iVar19,local_1b4);
    puVar16 = (undefined8 *)FUN_18009af60(local_180,&local_150,local_198,&local_c8);
    local_100 = *puVar16;
    uStack_f8 = puVar16[1];
    local_f0 = puVar16[2];
    uStack_e8 = puVar16[3];
    local_e0 = puVar16[4];
    uStack_d8 = puVar16[5];
    lVar8 = param_1[1];
    if (lVar8 == param_1[2]) {
      FUN_18005fdc0(param_1,lVar8,&local_128);
    }
    else {
      FUN_180017eb0(lVar8,&local_128);
      *(undefined4 *)(lVar8 + 0x20) = (undefined4)local_108;
      *(undefined4 *)(lVar8 + 0x24) = local_108._4_4_;
      *(undefined8 *)(lVar8 + 0x28) = local_100;
      *(undefined8 *)(lVar8 + 0x30) = uStack_f8;
      *(undefined8 *)(lVar8 + 0x38) = local_f0;
      *(undefined8 *)(lVar8 + 0x40) = uStack_e8;
      *(undefined8 *)(lVar8 + 0x48) = local_e0;
      *(undefined8 *)(lVar8 + 0x50) = uStack_d8;
      param_1[1] = param_1[1] + 0x58;
    }
    cVar13 = '\0';
    local_1b8 = '\0';
    local_1b4 = iVar19;
    if (0xf < local_110) {
      pvVar17 = local_128;
      if ((0xfff < local_110 + 1) &&
         (pvVar17 = *(void **)((longlong)local_128 + -8),
         0x1f < (ulonglong)((longlong)local_128 + (-8 - (longlong)pvVar17)))) {
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(pvVar17);
    }
    if (0xf < local_90) {
      pvVar2 = (void *)CONCAT71(uStack_a7,local_a8);
      pvVar17 = pvVar2;
      if ((0xfff < local_90 + 1) &&
         (pvVar17 = *(void **)((longlong)pvVar2 + -8),
         0x1f < (ulonglong)((longlong)pvVar2 + (-8 - (longlong)pvVar17)))) goto LAB_180060f5e;
      free(pvVar17);
    }
    local_98 = 0;
    local_90 = 0xf;
    local_a8 = 0;
    if (0xf < local_b0) {
      pvVar2 = (void *)CONCAT71(uStack_c7,local_c8);
      pvVar17 = pvVar2;
      if ((0xfff < local_b0 + 1) &&
         (pvVar17 = *(void **)((longlong)pvVar2 + -8),
         0x1f < (ulonglong)((longlong)pvVar2 + (-8 - (longlong)pvVar17)))) {
LAB_180060f5e:
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(pvVar17);
    }
    local_b8 = 0;
    local_b0 = 0xf;
    local_c8 = 0;
    iVar3 = iVar19;
    if (0xf < local_138) {
      pppppppuVar11 = local_150;
      if ((0xfff < local_138 + 1) &&
         (pppppppuVar11 = (undefined8 *******)local_150[-1],
         0x1f < (ulonglong)((longlong)local_150 + (-8 - (longlong)pppppppuVar11)))) {
LAB_180060f65:
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(pppppppuVar11);
    }
  } while( true );
}



## confidence

observed

## authority

shipped-artifact

## environment

null

## limitations

The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.,The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.,Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.,Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.,Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.,External functions and functions without an analyzable body return null; other decompiler failures remain explicit.,Ghidra Listing reports a terminal call at 0x180060f50 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180060f57 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180060f5e to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180060f65 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180060feb to 0x18001ae90 FUN_18001ae90 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180060ff1 to 0x18001a930 FUN_18001a930 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180060ff7 to 0x18001a930 FUN_18001a930 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180060ffd to 0x18001ae90 FUN_18001ae90 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Windows Ghidra P0 accepts native x86, x86-64, ARMNT (Thumb-2) and ARM64 PE applications and DLLs, and explicitly selected raw 6502 images; managed, hostile, sensitive, and mutable-path targets are unsupported.,The Windows bridge uses authenticated IPv4 loopback because Node path-based IPC does not expose Java AF_UNIX sockets; the endpoint file contains no bearer token.,Windows sessions require the matching packaged Windows x64 native addon, local NTFS targets and runtimes, and Windows 10 or later. Native handles enforce path admission, private DACLs, and Job Object ownership automatically.

## relationship_coverage

null

## locations



## evidence_links

