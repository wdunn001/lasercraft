## evidence_id

ev_b006f32abdf3df75d47f0464e49c233d0543c46b99760c4ee8623d754ba63615

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
| procedure | 0x180034e85 |

## raw_result.value


/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_180034dd0(longlong *param_1)

{
  undefined1 auVar1 [16];
  undefined1 *puVar2;
  void *pvVar3;
  bool bVar4;
  size_t _Size;
  char cVar5;
  int iVar6;
  int iVar7;
  undefined8 *puVar8;
  double *pdVar9;
  undefined8 uVar10;
  longlong *plVar11;
  char ****ppppcVar12;
  ulonglong uVar13;
  undefined8 ****ppppuVar14;
  undefined1 *puVar15;
  void *pvVar16;
  longlong *********ppppppppplVar17;
  char *****pppppcVar18;
  longlong *********ppppppppplVar19;
  char *pcVar20;
  char *****pppppcVar21;
  ulonglong uVar22;
  undefined8 *****pppppuVar23;
  longlong *********_Buf1;
  longlong lVar24;
  ulonglong uVar25;
  undefined8 *****pppppuVar26;
  undefined8 *****pppppuVar27;
  longlong ********pppppppplVar28;
  uint uVar29;
  uint uVar30;
  double dVar31;
  undefined1 auStack_1d8 [32];
  longlong *******local_1b8;
  longlong ********local_1b0;
  longlong *******local_1a8;
  char local_198 [8];
  undefined8 local_190;
  double local_188;
  longlong ********local_180;
  longlong *local_178 [2];
  undefined8 ****local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  ulonglong uStack_150;
  char ****local_148;
  longlong lStack_140;
  undefined1 local_138 [16];
  undefined8 ****local_128;
  undefined8 uStack_120;
  undefined1 local_118 [16];
  longlong ********local_108;
  longlong ********pppppppplStack_100;
  longlong *******local_f8;
  ulonglong uStack_f0;
  undefined8 ****local_e8;
  undefined8 uStack_e0;
  double local_d8;
  ulonglong uStack_d0;
  char ****local_c8;
  undefined8 uStack_c0;
  size_t local_b8;
  ulonglong local_b0;
  undefined8 ****local_a8;
  char ***pppcStack_a0;
  undefined1 local_98 [16];
  undefined1 local_88;
  undefined7 uStack_87;
  longlong local_78;
  ulonglong local_70;
  undefined1 local_68 [32];
  ulonglong local_48;
  
  local_48 = DAT_1803386c0 ^ (ulonglong)auStack_1d8;
  local_190 = (double)((ulonglong)local_190 & 0xffffffff00000000);
  local_178[0] = param_1;
  if (0x7fffffffffffffffU - (longlong)DAT_180338010 < 10) {
                    /* WARNING: Subroutine does not return */
    FUN_18001ae90();
  }
  local_1b0 = (longlong ********)&DAT_180338000;
  if (0xf < DAT_180338018) {
    local_1b0 = (longlong ********)DAT_180338000;
  }
  local_1a8 = (longlong *******)DAT_180338010;
  local_1b8 = (longlong *******)0xa;
  FUN_180018010(&local_a8,local_198[0],&DAT_180338000,";Version: ");
  puVar8 = (undefined8 *)FUN_18001bfc0(&local_a8,"\n;Start GCode\nG90\n",0x12);
  local_e8 = (undefined8 ****)*puVar8;
  uStack_e0 = puVar8[1];
  local_d8 = (double)puVar8[2];
  uStack_d0 = puVar8[3];
  puVar8[2] = 0;
  puVar8[3] = 0xf;
  *(undefined1 *)puVar8 = 0;
  FUN_180018de0(&local_a8);
  local_158 = 0xc;
  uStack_150 = 0xf;
  local_168 = (undefined8 ****)0x735f676f6a5f7978;
  uStack_160 = 0x64656570;
  dVar31 = (double)FUN_180023f50(param_1,&local_168);
  local_118._8_8_ = 0xf;
  local_118._0_8_ = 10;
  local_128 = (undefined8 ****)0x6570735f6b726f77;
  uStack_120 = 0x6465;
  local_190 = dVar31;
  local_188 = (double)FUN_180023f50(param_1,&local_128);
  pdVar9 = (double *)&local_190;
  if (dVar31 <= local_188) {
    pdVar9 = &local_188;
  }
  uVar10 = FUN_180045890(local_68,*pdVar9);
  uVar10 = FUN_18001ffb0(&local_108,"G1 U0 V0 W0 F",uVar10);
  plVar11 = (longlong *)FUN_18001bfc0(uVar10,&DAT_18028e194,1);
  local_148 = (char ****)*plVar11;
  lStack_140 = plVar11[1];
  lVar24 = *(longlong *)*(undefined1 (*) [16])(plVar11 + 2);
  uVar22 = plVar11[3];
  local_138 = *(undefined1 (*) [16])(plVar11 + 2);
  plVar11[2] = 0;
  plVar11[3] = 0xf;
  *(undefined1 *)plVar11 = 0;
  uVar29 = 0xe00;
  pppppcVar21 = &local_148;
  if (0xf < uVar22) {
    pppppcVar21 = (char *****)local_148;
  }
  FUN_18001bfc0(&local_e8,pppppcVar21,lVar24);
  if (0xf < (ulonglong)local_138._8_8_) {
    pppppcVar21 = (char *****)local_148;
    if ((0xfff < local_138._8_8_ + 1) &&
       (pppppcVar21 = (char *****)local_148[-1],
       0x1f < (ulonglong)((longlong)local_148 + (-8 - (longlong)pppppcVar21)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pppppcVar21);
  }
  FUN_180018de0(&local_108);
  FUN_180018de0(local_68);
  if (0xf < (ulonglong)local_118._8_8_) {
    FUN_180019d40(&local_128,local_128);
  }
  local_118 = ZEXT816(0xf) << 0x40;
  local_128 = (undefined8 ****)((ulonglong)local_128 & 0xffffffffffffff00);
  if (0xf < uStack_150) {
    FUN_180019d40(&local_168,local_168);
  }
  local_190 = (double)CONCAT44(local_190._4_4_,0xffffffff);
  local_158 = 8;
  uStack_150 = 0xf;
  local_168 = (undefined8 ****)0x64695f726573616c;
  uStack_160 = 0;
  cVar5 = FUN_180055f90(param_1,&local_168);
  if (cVar5 == '\0') {
    if (*param_1 == 0) {
      iVar6 = (int)local_190;
    }
    else {
      iVar6 = FUN_180023c70(*param_1,&local_168,&local_190);
    }
  }
  else {
    iVar6 = FUN_180023b10(param_1,&local_168);
  }
  if (0xf < uStack_150) {
    ppppuVar14 = local_168;
    if ((0xfff < uStack_150 + 1) &&
       (ppppuVar14 = (undefined8 ****)local_168[-1],
       0x1f < (ulonglong)((longlong)local_168 + (-8 - (longlong)ppppuVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(ppppuVar14);
  }
  if (iVar6 != -1) {
    uVar10 = FUN_1800457c0(&local_168,iVar6);
    uVar10 = FUN_18001ffb0(local_68,&DAT_18028e198,uVar10);
    plVar11 = (longlong *)FUN_18001bfc0(uVar10,&DAT_18028e194,1);
    local_a8 = (undefined8 ****)*plVar11;
    pppcStack_a0 = (char ***)plVar11[1];
    lVar24 = *(longlong *)*(undefined1 (*) [16])(plVar11 + 2);
    uVar22 = plVar11[3];
    local_98 = *(undefined1 (*) [16])(plVar11 + 2);
    plVar11[2] = 0;
    plVar11[3] = 0xf;
    *(undefined1 *)plVar11 = 0;
    uVar29 = 0x1e00;
    pppppuVar26 = &local_a8;
    if (0xf < uVar22) {
      pppppuVar26 = (undefined8 *****)local_a8;
    }
    FUN_18001bfc0(&local_e8,pppppuVar26,lVar24);
    FUN_180018de0(&local_a8);
    FUN_180018de0(local_68);
    FUN_180018de0(&local_168);
  }
  uStack_160 = 0;
  local_158 = 4;
  uStack_150 = 0xf;
  local_168 = (undefined8 ****)0x656e6f6e;
  local_118 = _DAT_18028c630;
  local_128 = (undefined8 ****)&DAT_5f676e696b726f77;
  uStack_120 = 0x65646f6d5f7a;
  uVar30 = uVar29 | 0x100;
  cVar5 = FUN_180055f90(param_1,&local_128);
  if (cVar5 == '\0') {
    if (*param_1 == 0) {
      FUN_180017eb0(&local_88,&local_168);
    }
    else {
      FUN_1800243c0(*param_1,&local_88,&local_128,&local_168);
    }
  }
  else {
    FUN_180024440(param_1,&local_88,&local_128);
  }
  if (0xf < (ulonglong)local_118._8_8_) {
    ppppuVar14 = local_128;
    if ((0xfff < local_118._8_8_ + 1) &&
       (ppppuVar14 = (undefined8 ****)local_128[-1],
       (undefined1 *)0x1f < (undefined1 *)((longlong)local_128 + (-8 - (longlong)ppppuVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(ppppuVar14);
  }
  local_118 = ZEXT816(0xf) << 0x40;
  local_128 = (undefined8 ****)((ulonglong)local_128 & 0xffffffffffffff00);
  if (0xf < uStack_150) {
    ppppuVar14 = local_168;
    if ((0xfff < uStack_150 + 1) &&
       (ppppuVar14 = (undefined8 ****)local_168[-1],
       0x1f < (ulonglong)((longlong)local_168 + (-8 - (longlong)ppppuVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(ppppuVar14);
  }
  uVar22 = local_70;
  local_158 = 0;
  uStack_150 = 0xf;
  local_168 = (undefined8 ****)((ulonglong)local_168 & 0xffffffffffffff00);
  puVar2 = (undefined1 *)CONCAT71(uStack_87,local_88);
  puVar15 = &local_88;
  if (0xf < local_70) {
    puVar15 = puVar2;
  }
  if ((local_78 == 6) && (iVar7 = memcmp(puVar15,"manual",6), iVar7 == 0)) {
    FUN_18001bfc0(&local_e8,&DAT_18028efb0,4);
    local_190 = 0.0;
    local_f8 = (longlong *******)0x9;
    uStack_f0 = 0xf;
    local_108 = (longlong ********)&DAT_5f676e696b726f77;
    pppppppplStack_100 = (longlong ********)0x7a;
    cVar5 = FUN_180055f90(param_1,&local_108);
    if (cVar5 == '\0') {
      if (*param_1 != 0) {
        FUN_1800240d0(*param_1,&local_108,&local_190);
      }
    }
    else {
      FUN_180023f50(param_1,&local_108);
    }
    FUN_180029cd0(&local_e8);
    if (0xf < uStack_f0) {
      FUN_180019d40(&local_108,local_108);
    }
    FUN_18001bfc0(&local_e8,&DAT_18028efc4,2);
    local_138._8_8_ = 0xf;
    local_138._0_8_ = 0xc;
    local_148 = (char ****)0x735f676f6a5f7978;
    lStack_140 = 0x64656570;
    FUN_180023f50(param_1,&local_148);
    FUN_180029cd0(&local_e8);
    FUN_180018de0(&local_148);
    uVar10 = 1;
    pcVar20 = "\n";
LAB_1800354b5:
    FUN_18001bfc0(&local_e8,pcVar20,uVar10);
  }
  else {
    puVar15 = &local_88;
    if (0xf < uVar22) {
      puVar15 = puVar2;
    }
    if ((local_78 == 5) && (iVar7 = memcmp(puVar15,"lidar",5), iVar7 == 0)) {
      uVar10 = 9;
      pcVar20 = "LR_FOCUS\n";
      goto LAB_1800354b5;
    }
  }
  if (iVar6 == 1) {
    local_198[0] = '\0';
    local_138 = _DAT_18028fe90;
    local_148 = (char ****)0x616e655f61706f6d;
    lStack_140 = 0x64656c62;
    uVar30 = uVar29 | 0x101;
    local_190 = (double)(CONCAT44(local_190._4_4_,uVar29) | 0x101);
    cVar5 = FUN_180055f90(param_1,&local_148);
    if (cVar5 == '\0') {
      cVar5 = local_198[0];
      if (*param_1 != 0) {
        cVar5 = FUN_180025360(*param_1,&local_148,local_198);
      }
    }
    else {
      cVar5 = FUN_180025200(param_1,&local_148);
    }
    if (cVar5 == '\0') goto LAB_180035541;
    bVar4 = true;
  }
  else {
LAB_180035541:
    bVar4 = false;
  }
  if ((uVar30 & 1) != 0) {
    uVar30 = uVar30 & 0xfffffffe;
    FUN_180018de0(&local_148);
  }
  if (bVar4) {
    FUN_18001bfc0(&local_e8,"MOPA_SET_MO MO=1\n",0x11);
  }
  FUN_1800365d0(&local_108,param_1,1);
  cVar5 = local_198[0];
  local_180 = pppppppplStack_100;
  ppppppppplVar17 = (longlong *********)local_108;
  if (local_108 != pppppppplStack_100) {
    do {
      ppppppppplVar19 = ppppppppplVar17;
      if ((longlong ********)0xf < ppppppppplVar17[3]) {
        ppppppppplVar19 = (longlong *********)*ppppppppplVar17;
      }
      _Buf1 = ppppppppplVar19;
      if ((longlong ********)0xc < ppppppppplVar17[2]) {
        while( true ) {
          if ((*(char *)_Buf1 == 'W') && (iVar6 = memcmp(_Buf1,"WOBBLE_START ",0xd), iVar6 == 0)) {
            lVar24 = (longlong)_Buf1 - (longlong)ppppppppplVar19;
            goto LAB_1800355eb;
          }
          if (_Buf1 == ppppppppplVar19) break;
          _Buf1 = (longlong *********)((longlong)_Buf1 + -1);
        }
      }
      lVar24 = -1;
LAB_1800355eb:
      pppppppplVar28 = ppppppppplVar17[2];
      if (lVar24 == 0) {
        if (-(longlong)pppppppplVar28 + 0x7fffffffffffffffU < 5) {
                    /* WARNING: Subroutine does not return */
          FUN_18001ae90();
        }
        local_1b0 = (longlong ********)ppppppppplVar17;
        if ((longlong ********)0xf < ppppppppplVar17[3]) {
          local_1b0 = *ppppppppplVar17;
        }
        local_1b8 = (longlong *******)0x5;
        local_1a8 = (longlong *******)pppppppplVar28;
        FUN_180018010(&local_148,cVar5,ppppppppplVar17,"M400\n");
        uVar10 = local_138._0_8_;
        if ((ulonglong)(local_138._8_8_ - local_138._0_8_) < 10) {
          local_1b8 = (longlong *******)0xa;
          pppppcVar21 = (char *****)FUN_180016ae0(&local_148,10,0,"\nG00 P100\n");
        }
        else {
          local_138._0_8_ = local_138._0_8_ + 10;
          pppppcVar21 = &local_148;
          if (0xf < (ulonglong)local_138._8_8_) {
            pppppcVar21 = (char *****)local_148;
          }
          memmove((void *)((longlong)pppppcVar21 + uVar10),"\nG00 P100\n",10);
          *(undefined1 *)((longlong)pppppcVar21 + uVar10 + 10) = 0;
          pppppcVar21 = &local_148;
        }
        pppppuVar26 = (undefined8 *****)*pppppcVar21;
        local_a8 = pppppuVar26;
        pppcStack_a0 = (char ***)pppppcVar21[1];
        auVar1 = *(undefined1 (*) [16])(pppppcVar21 + 2);
        local_98 = auVar1;
        pppppcVar21[2] = (char ****)0x0;
        pppppcVar21[3] = (char ****)0xf;
        *(undefined1 *)pppppcVar21 = 0;
        uVar30 = uVar30 | 0x18;
        local_190 = auVar1._8_8_;
        pppppuVar27 = &local_a8;
        if (0xf < (ulonglong)local_190) {
          pppppuVar27 = pppppuVar26;
        }
        local_188 = local_d8;
        pppppppplVar28 = auVar1._0_8_;
        if ((longlong ********)(uStack_d0 - (longlong)local_d8) < pppppppplVar28) {
          local_1b8 = (longlong *******)pppppppplVar28;
          FUN_180016ae0(&local_e8,pppppppplVar28,0);
          dVar31 = (double)local_98._8_8_;
          pppppuVar26 = (undefined8 *****)local_a8;
        }
        else {
          pppppuVar23 = &local_e8;
          if (0xf < uStack_d0) {
            pppppuVar23 = (undefined8 *****)local_e8;
          }
          pvVar16 = (void *)((longlong)local_d8 + (longlong)pppppuVar23);
          local_d8 = (double)((longlong)local_d8 + (longlong)pppppppplVar28);
          memmove(pvVar16,pppppuVar27,(size_t)pppppppplVar28);
          *(undefined1 *)((longlong)local_188 + (longlong)pppppuVar23 + (longlong)pppppppplVar28) =
               0;
          dVar31 = local_190;
        }
        if (0xf < (ulonglong)dVar31) {
          pppppuVar27 = pppppuVar26;
          if ((0xfff < (longlong)dVar31 + 1U) &&
             (pppppuVar27 = (undefined8 *****)pppppuVar26[-1],
             0x1f < (ulonglong)((longlong)pppppuVar26 + (-8 - (longlong)pppppuVar27)))) {
                    /* WARNING: Subroutine does not return */
            _invalid_parameter_noinfo_noreturn();
          }
          free(pppppuVar27);
        }
        if (0xf < (ulonglong)local_138._8_8_) {
          FUN_180019d40(&local_148,local_148);
        }
      }
      else {
        if (-(longlong)pppppppplVar28 == -0x7fffffffffffffff) {
                    /* WARNING: Subroutine does not return */
          FUN_18001ae90();
        }
        ppppppppplVar19 = ppppppppplVar17;
        if ((longlong ********)0xf < ppppppppplVar17[3]) {
          ppppppppplVar19 = (longlong *********)*ppppppppplVar17;
        }
        local_1a8 = (longlong *******)0x1;
        local_1b0 = (longlong ********)&DAT_18028e194;
        local_1b8 = (longlong *******)pppppppplVar28;
        FUN_180018010(&local_148,cVar5,ppppppppplVar17,ppppppppplVar19);
        pppppcVar21 = (char *****)local_148;
        uVar30 = uVar30 | 0x40;
        pppppcVar18 = &local_148;
        if (0xf < (ulonglong)local_138._8_8_) {
          pppppcVar18 = (char *****)local_148;
        }
        uVar10 = local_138._0_8_;
        if ((longlong ********)(uStack_d0 - (longlong)local_d8) < (ulonglong)local_138._0_8_) {
          local_1b8 = (longlong *******)local_138._0_8_;
          FUN_180016ae0(&local_e8,local_138._0_8_,0);
          pppppcVar21 = (char *****)local_148;
        }
        else {
          pppppuVar26 = &local_e8;
          if (0xf < uStack_d0) {
            pppppuVar26 = (undefined8 *****)local_e8;
          }
          pvVar16 = (void *)((longlong)pppppuVar26 + (longlong)local_d8);
          local_d8 = (double)((longlong)local_d8 + local_138._0_8_);
          memmove(pvVar16,pppppcVar18,local_138._0_8_);
          *(undefined1 *)((longlong)pvVar16 + uVar10) = 0;
        }
        if (0xf < (ulonglong)local_138._8_8_) {
          pppppcVar18 = pppppcVar21;
          if ((0xfff < local_138._8_8_ + 1) &&
             (pppppcVar18 = (char *****)pppppcVar21[-1],
             0x1f < (ulonglong)((longlong)pppppcVar21 + (-8 - (longlong)pppppcVar18)))) {
                    /* WARNING: Subroutine does not return */
            _invalid_parameter_noinfo_noreturn();
          }
          free(pppppcVar18);
        }
      }
      ppppppppplVar17 = ppppppppplVar17 + 4;
      param_1 = local_178[0];
    } while (ppppppppplVar17 != (longlong *********)local_180);
  }
  if ((longlong *********)local_108 != (longlong *********)0x0) {
    FUN_180015de0(local_108,pppppppplStack_100,&local_108);
    ppppppppplVar17 = (longlong *********)local_108;
    if ((0xfff < ((longlong)local_f8 - (longlong)local_108 & 0xffffffffffffffe0U)) &&
       (ppppppppplVar17 = (longlong *********)local_108[-1],
       (char *)0x1f < (char *)((longlong)local_108 + (-8 - (longlong)ppppppppplVar17)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(ppppppppplVar17);
  }
  local_c8 = (char ****)0x0;
  uStack_c0 = 0;
  pppppcVar21 = (char *****)0x0;
  local_b8 = 0;
  local_b0 = 0;
  local_c8 = (char ****)FUN_18024833c(0x20);
  local_b8 = 0x15;
  local_b0 = 0x1f;
  *local_c8 = (char ***)0x5f5245444f434e45;
  local_c8[1] = (char ***)0x414e45204e525554;
  *(undefined4 *)(local_c8 + 2) = 0x3d454c42;
  *(char *)((longlong)local_c8 + 0x14) = '0';
  *(char *)((longlong)local_c8 + 0x15) = '\0';
  cVar5 = FUN_18004c9f0(param_1);
  uVar22 = local_b0;
  if (cVar5 != '\0') {
    if (local_b0 < 0x1f) {
      if (0x7fffffffffffffff - (local_b0 >> 1) < local_b0) {
        uVar25 = 0x7fffffffffffffff;
        uVar13 = 0x8000000000000027;
LAB_1800359ee:
        ppppcVar12 = (char ****)FUN_18024833c(uVar13);
        if (ppppcVar12 == (char ****)0x0) goto LAB_180035ddb;
        pppppcVar21 = (char *****)((longlong)ppppcVar12 + 0x27U & 0xffffffffffffffe0);
        pppppcVar21[-1] = ppppcVar12;
      }
      else {
        uVar13 = (local_b0 >> 1) + local_b0;
        uVar25 = 0x1f;
        if (0x1f < uVar13) {
          uVar25 = uVar13;
        }
        if (0xfff < uVar25 + 1) {
          uVar13 = uVar25 + 0x28;
          if (uVar13 <= uVar25 + 1) {
                    /* WARNING: Subroutine does not return */
            FUN_18001a930();
          }
          goto LAB_1800359ee;
        }
        if (uVar25 != 0xffffffffffffffff) {
          pppppcVar21 = (char *****)FUN_18024833c();
        }
      }
      local_b8 = 0x1f;
      *pppppcVar21 = (char ****)0x5f5245444f434e45;
      pppppcVar21[1] = (char ****)0x414e45204e525554;
      pppppcVar21[2] = (char ****)0x535520313d454c42;
      *(undefined4 *)(pppppcVar21 + 3) = 0x414d5f45;
      ((char *)((longlong)pppppcVar21 + 0x1c))[0] = 'P';
      ((char *)((longlong)pppppcVar21 + 0x1c))[1] = '=';
      *(char *)((longlong)pppppcVar21 + 0x1e) = '1';
      *(char *)((longlong)pppppcVar21 + 0x1f) = '\0';
      local_b0 = uVar25;
      if (0xf < uVar22) {
        pppppcVar18 = (char *****)local_c8;
        if ((0xfff < uVar22 + 1) &&
           (pppppcVar18 = (char *****)local_c8[-1],
           (char *)0x1f < (char *)((longlong)local_c8 + (-8 - (longlong)pppppcVar18))))
        goto LAB_180035ddb;
        free(pppppcVar18);
      }
    }
    else {
      pppppcVar21 = &local_c8;
      if (0xf < local_b0) {
        pppppcVar21 = (char *****)local_c8;
      }
      local_b8 = 0x1f;
      memmove(pppppcVar21,"ENCODER_TURN ENABLE=1 USE_MAP=1",0x1f);
      *(char *)((longlong)pppppcVar21 + 0x1f) = '\0';
      pppppcVar21 = (char *****)local_c8;
    }
    local_c8 = (char ****)pppppcVar21;
    uStack_120 = 0;
    local_118 = ZEXT816(0xf) << 0x40;
    local_128 = (undefined8 ****)0x0;
    local_148 = (char ****)0x0;
    lStack_140 = 0;
    local_138 = ZEXT816(0);
    local_148 = (char ****)FUN_18024833c(0x20);
    local_138._8_8_ = 0x1f;
    local_138._0_8_ = 0x11;
    *local_148 = (char ***)0x5f7265646f636e65;
    local_148[1] = (char ***)0x6978615f6e727574;
    *(char *)(local_148 + 2) = 's';
    *(char *)((longlong)local_148 + 0x11) = '\0';
    cVar5 = FUN_180055f90(param_1,&local_148);
    if (cVar5 == '\0') {
      if (*param_1 == 0) {
        FUN_180017eb0(&local_108,&local_128);
      }
      else {
        FUN_1800243c0(*param_1,&local_108,&local_148,&local_128);
      }
    }
    else {
      FUN_180024440(param_1,&local_108,&local_148);
    }
    if (0xf < (ulonglong)local_138._8_8_) {
      ppppcVar12 = local_148;
      if ((0xfff < local_138._8_8_ + 1) &&
         (ppppcVar12 = (char ****)local_148[-1],
         (char *)0x1f < (char *)((longlong)local_148 + (-8 - (longlong)ppppcVar12)))) {
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(ppppcVar12);
    }
    local_138 = ZEXT816(0xf) << 0x40;
    local_148 = (char ****)((ulonglong)local_148 & 0xffffffffffffff00);
    if (0xf < (ulonglong)local_118._8_8_) {
      ppppuVar14 = local_128;
      if ((0xfff < local_118._8_8_ + 1) &&
         (ppppuVar14 = (undefined8 ****)local_128[-1],
         0x1f < (ulonglong)((longlong)local_128 + (-8 - (longlong)ppppuVar14)))) {
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(ppppuVar14);
    }
    local_118 = ZEXT816(0xf) << 0x40;
    local_128 = (undefined8 ****)((ulonglong)local_128 & 0xffffffffffffff00);
    local_158 = 0xf;
    uStack_150 = 0xf;
    local_168 = (undefined8 *****)0x676e696e6e616c70;
    uStack_160 = 0x646f6874656d5f;
    local_190 = (double)(CONCAT44(local_190._4_4_,uVar30) | 0x22);
    iVar6 = FUN_180024d10(param_1,&local_168);
    uVar22 = uStack_f0;
    pppppppplVar28 = (longlong ********)local_f8;
    ppppppppplVar17 = (longlong *********)local_108;
    if (iVar6 == 10) {
      ppppppppplVar19 = &local_108;
      if (0xf < uStack_f0) {
        ppppppppplVar19 = (longlong *********)local_108;
      }
      if (((longlong ********)local_f8 != (longlong ********)0x1) ||
         (iVar6 = memcmp(ppppppppplVar19,&DAT_18028e1a0,1), iVar6 != 0)) {
        ppppppppplVar19 = &local_108;
        if (0xf < uVar22) {
          ppppppppplVar19 = ppppppppplVar17;
        }
        if ((pppppppplVar28 != (longlong ********)0x1) ||
           (iVar6 = memcmp(ppppppppplVar19,&DAT_18028e19c,1), iVar6 != 0)) goto LAB_180035cca;
      }
      bVar4 = true;
    }
    else {
LAB_180035cca:
      bVar4 = false;
    }
    if (0xf < uStack_150) {
      pppppuVar26 = (undefined8 *****)local_168;
      if ((0xfff < uStack_150 + 1) &&
         (pppppuVar26 = (undefined8 *****)local_168[-1],
         0x1f < (ulonglong)((longlong)local_168 + (-8 - (longlong)pppppuVar26)))) {
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(pppppuVar26);
      ppppppppplVar17 = (longlong *********)local_108;
      uVar22 = uStack_f0;
      pppppppplVar28 = (longlong ********)local_f8;
    }
    if (bVar4) {
      if (0x7fffffffffffffffU - (longlong)pppppppplVar28 < 6) {
                    /* WARNING: Subroutine does not return */
        FUN_18001ae90();
      }
      local_1b0 = (longlong ********)&local_108;
      if (0xf < uVar22) {
        local_1b0 = (longlong ********)ppppppppplVar17;
      }
      local_1b8 = (longlong *******)0x6;
      local_1a8 = (longlong *******)pppppppplVar28;
      FUN_180018010(&local_168,local_198[0],&local_108," AXIS=");
      pppppuVar26 = &local_168;
      if (0xf < uStack_150) {
        pppppuVar26 = (undefined8 *****)local_168;
      }
      FUN_18001bfc0(&local_c8,pppppuVar26,local_158);
      ppppppppplVar17 = (longlong *********)local_108;
      uVar22 = uStack_f0;
      if (0xf < uStack_150) {
        FUN_180019d40(&local_168,local_168);
        ppppppppplVar17 = (longlong *********)local_108;
        uVar22 = uStack_f0;
      }
    }
    if (0xf < uVar22) {
      ppppppppplVar19 = ppppppppplVar17;
      if ((0xfff < uVar22 + 1) &&
         (ppppppppplVar19 = (longlong *********)ppppppppplVar17[-1],
         (char *)0x1f < (char *)((longlong)ppppppppplVar17 + (-8 - (longlong)ppppppppplVar19)))) {
LAB_180035ddb:
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(ppppppppplVar19);
    }
  }
  _Size = local_b8;
  if (local_b8 == 0x7fffffffffffffff) {
                    /* WARNING: Subroutine does not return */
    FUN_18001ae90();
  }
  pppppcVar21 = &local_c8;
  if (0xf < local_b0) {
    pppppcVar21 = (char *****)local_c8;
  }
  local_128 = (undefined8 *****)0x0;
  uStack_120 = 0;
  local_118._0_8_ = 0;
  uVar10 = local_118._0_8_;
  local_118._0_8_ = (longlong ********)0x0;
  local_118._8_8_ = 0;
  pppppppplVar28 = (longlong ********)(local_b8 + 1);
  uVar22 = 0xf;
  pppppuVar26 = &local_128;
  if ((longlong ********)0xf < pppppppplVar28) {
    uVar22 = (ulonglong)pppppppplVar28 | 0xf;
    if (uVar22 < 0x8000000000000000) {
      if (uVar22 < 0x16) {
        uVar22 = 0x16;
      }
      if (uVar22 + 1 < 0x1000) {
        if (uVar22 == 0xffffffffffffffff) {
          pppppuVar26 = (undefined8 *****)0x0;
          local_128 = pppppuVar26;
        }
        else {
          pppppuVar26 = (undefined8 *****)FUN_18024833c();
          local_128 = pppppuVar26;
        }
        goto LAB_180035ea6;
      }
      uVar13 = uVar22 + 0x28;
      if (uVar13 <= uVar22 + 1) {
        local_118._0_8_ = uVar10;
                    /* WARNING: Subroutine does not return */
        FUN_18001a930();
      }
    }
    else {
      uVar22 = 0x7fffffffffffffff;
      uVar13 = 0x8000000000000027;
    }
    ppppuVar14 = (undefined8 ****)FUN_18024833c(uVar13);
    if (ppppuVar14 == (undefined8 ****)0x0) goto LAB_180035f67;
    pppppuVar26 = (undefined8 *****)((longlong)ppppuVar14 + 0x27U & 0xffffffffffffffe0);
    pppppuVar26[-1] = ppppuVar14;
    local_128 = pppppuVar26;
  }
LAB_180035ea6:
  local_118._0_8_ = pppppppplVar28;
  local_118._8_8_ = uVar22;
  memcpy(pppppuVar26,pppppcVar21,_Size);
  *(undefined1 *)((longlong)pppppuVar26 + _Size) = 10;
  *(undefined1 *)((longlong)pppppuVar26 + (longlong)pppppppplVar28) = 0;
  dVar31 = local_d8;
  uVar22 = local_118._8_8_;
  uVar10 = local_118._0_8_;
  pppppuVar26 = (undefined8 *****)local_128;
  pppppuVar27 = &local_128;
  if (0xf < (ulonglong)local_118._8_8_) {
    pppppuVar27 = (undefined8 *****)local_128;
  }
  if ((longlong ********)(uStack_d0 - (longlong)local_d8) < (ulonglong)local_118._0_8_) {
    local_1b8 = (longlong *******)local_118._0_8_;
    FUN_180016ae0(&local_e8,local_118._0_8_,0);
    pppppuVar26 = (undefined8 *****)local_128;
    uVar22 = local_118._8_8_;
  }
  else {
    pppppuVar23 = &local_e8;
    if (0xf < uStack_d0) {
      pppppuVar23 = (undefined8 *****)local_e8;
    }
    pvVar16 = (void *)((longlong)pppppuVar23 + (longlong)local_d8);
    local_d8 = (double)((longlong)local_d8 + local_118._0_8_);
    memmove(pvVar16,pppppuVar27,local_118._0_8_);
    *(undefined1 *)(uVar10 + (longlong)pppppuVar23 + (longlong)dVar31) = 0;
  }
  if (0xf < uVar22) {
    pppppuVar27 = pppppuVar26;
    if ((0xfff < uVar22 + 1) &&
       (pppppuVar27 = (undefined8 *****)pppppuVar26[-1],
       0x1f < (ulonglong)((longlong)pppppuVar26 + (-8 - (longlong)pppppuVar27)))) {
LAB_180035f67:
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pppppuVar27);
  }
  local_158 = 0xb;
  uStack_150 = 0xf;
  local_168 = (undefined8 ****)0x63675f7472617473;
  uStack_160 = 0x65646f;
  uVar10 = FUN_1800536b0(param_1,&local_a8,&local_e8);
  plVar11 = (longlong *)FUN_180017120(param_1 + 2,local_178,&local_168);
  FUN_180019770(*plVar11 + 0x30,uVar10);
  if (0xf < (ulonglong)local_98._8_8_) {
    pppppuVar26 = (undefined8 *****)local_a8;
    if ((0xfff < local_98._8_8_ + 1) &&
       (pppppuVar26 = (undefined8 *****)local_a8[-1],
       0x1f < (ulonglong)((longlong)local_a8 + (-8 - (longlong)pppppuVar26)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pppppuVar26);
  }
  if (0xf < uStack_150) {
    ppppuVar14 = local_168;
    if ((0xfff < uStack_150 + 1) &&
       (ppppuVar14 = (undefined8 ****)local_168[-1],
       0x1f < (ulonglong)((longlong)local_168 + (-8 - (longlong)ppppuVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(ppppuVar14);
  }
  if (0xf < local_b0) {
    pppppcVar21 = (char *****)local_c8;
    if ((0xfff < local_b0 + 1) &&
       (pppppcVar21 = (char *****)local_c8[-1],
       (char *)0x1f < (char *)((longlong)local_c8 + (-8 - (longlong)pppppcVar21)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pppppcVar21);
  }
  local_b8 = 0;
  local_b0 = 0xf;
  local_c8 = (char ****)((ulonglong)local_c8 & 0xffffffffffffff00);
  if (0xf < local_70) {
    pvVar3 = (void *)CONCAT71(uStack_87,local_88);
    pvVar16 = pvVar3;
    if ((0xfff < local_70 + 1) &&
       (pvVar16 = *(void **)((longlong)pvVar3 + -8),
       0x1f < (ulonglong)((longlong)pvVar3 + (-8 - (longlong)pvVar16)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pvVar16);
  }
  local_78 = 0;
  local_70 = 0xf;
  local_88 = 0;
  if (0xf < uStack_d0) {
    pppppuVar26 = (undefined8 *****)local_e8;
    if ((0xfff < uStack_d0 + 1) &&
       (pppppuVar26 = (undefined8 *****)local_e8[-1],
       0x1f < (ulonglong)((longlong)local_e8 + (-8 - (longlong)pppppuVar26)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pppppuVar26);
  }
  return;
}



## raw_result.limitations

Ghidra Listing reports a terminal call at 0x180035023 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003512a to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x1800352bd to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003530d to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180035918 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003591f to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180035926 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180035b9f to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180035bee to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180035d06 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180035ddb to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180035f67 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003601d to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003605d to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003609c to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x1800360eb to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036139 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003617a to 0x18001ae90 FUN_18001ae90 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036180 to 0x18001a930 FUN_18001a930 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036186 to 0x18001a930 FUN_18001a930 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003618c to 0x18001ae90 FUN_18001ae90 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036192 to 0x18001ae90 FUN_18001ae90 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036198 to 0x18001ae90 FUN_18001ae90 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003619e to 0x18001ae90 FUN_18001ae90 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.

## normalized_result


/* WARNING: Function: __security_check_cookie replaced with injection: security_check_cookie */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_180034dd0(longlong *param_1)

{
  undefined1 auVar1 [16];
  undefined1 *puVar2;
  void *pvVar3;
  bool bVar4;
  size_t _Size;
  char cVar5;
  int iVar6;
  int iVar7;
  undefined8 *puVar8;
  double *pdVar9;
  undefined8 uVar10;
  longlong *plVar11;
  char ****ppppcVar12;
  ulonglong uVar13;
  undefined8 ****ppppuVar14;
  undefined1 *puVar15;
  void *pvVar16;
  longlong *********ppppppppplVar17;
  char *****pppppcVar18;
  longlong *********ppppppppplVar19;
  char *pcVar20;
  char *****pppppcVar21;
  ulonglong uVar22;
  undefined8 *****pppppuVar23;
  longlong *********_Buf1;
  longlong lVar24;
  ulonglong uVar25;
  undefined8 *****pppppuVar26;
  undefined8 *****pppppuVar27;
  longlong ********pppppppplVar28;
  uint uVar29;
  uint uVar30;
  double dVar31;
  undefined1 auStack_1d8 [32];
  longlong *******local_1b8;
  longlong ********local_1b0;
  longlong *******local_1a8;
  char local_198 [8];
  undefined8 local_190;
  double local_188;
  longlong ********local_180;
  longlong *local_178 [2];
  undefined8 ****local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  ulonglong uStack_150;
  char ****local_148;
  longlong lStack_140;
  undefined1 local_138 [16];
  undefined8 ****local_128;
  undefined8 uStack_120;
  undefined1 local_118 [16];
  longlong ********local_108;
  longlong ********pppppppplStack_100;
  longlong *******local_f8;
  ulonglong uStack_f0;
  undefined8 ****local_e8;
  undefined8 uStack_e0;
  double local_d8;
  ulonglong uStack_d0;
  char ****local_c8;
  undefined8 uStack_c0;
  size_t local_b8;
  ulonglong local_b0;
  undefined8 ****local_a8;
  char ***pppcStack_a0;
  undefined1 local_98 [16];
  undefined1 local_88;
  undefined7 uStack_87;
  longlong local_78;
  ulonglong local_70;
  undefined1 local_68 [32];
  ulonglong local_48;
  
  local_48 = DAT_1803386c0 ^ (ulonglong)auStack_1d8;
  local_190 = (double)((ulonglong)local_190 & 0xffffffff00000000);
  local_178[0] = param_1;
  if (0x7fffffffffffffffU - (longlong)DAT_180338010 < 10) {
                    /* WARNING: Subroutine does not return */
    FUN_18001ae90();
  }
  local_1b0 = (longlong ********)&DAT_180338000;
  if (0xf < DAT_180338018) {
    local_1b0 = (longlong ********)DAT_180338000;
  }
  local_1a8 = (longlong *******)DAT_180338010;
  local_1b8 = (longlong *******)0xa;
  FUN_180018010(&local_a8,local_198[0],&DAT_180338000,";Version: ");
  puVar8 = (undefined8 *)FUN_18001bfc0(&local_a8,"\n;Start GCode\nG90\n",0x12);
  local_e8 = (undefined8 ****)*puVar8;
  uStack_e0 = puVar8[1];
  local_d8 = (double)puVar8[2];
  uStack_d0 = puVar8[3];
  puVar8[2] = 0;
  puVar8[3] = 0xf;
  *(undefined1 *)puVar8 = 0;
  FUN_180018de0(&local_a8);
  local_158 = 0xc;
  uStack_150 = 0xf;
  local_168 = (undefined8 ****)0x735f676f6a5f7978;
  uStack_160 = 0x64656570;
  dVar31 = (double)FUN_180023f50(param_1,&local_168);
  local_118._8_8_ = 0xf;
  local_118._0_8_ = 10;
  local_128 = (undefined8 ****)0x6570735f6b726f77;
  uStack_120 = 0x6465;
  local_190 = dVar31;
  local_188 = (double)FUN_180023f50(param_1,&local_128);
  pdVar9 = (double *)&local_190;
  if (dVar31 <= local_188) {
    pdVar9 = &local_188;
  }
  uVar10 = FUN_180045890(local_68,*pdVar9);
  uVar10 = FUN_18001ffb0(&local_108,"G1 U0 V0 W0 F",uVar10);
  plVar11 = (longlong *)FUN_18001bfc0(uVar10,&DAT_18028e194,1);
  local_148 = (char ****)*plVar11;
  lStack_140 = plVar11[1];
  lVar24 = *(longlong *)*(undefined1 (*) [16])(plVar11 + 2);
  uVar22 = plVar11[3];
  local_138 = *(undefined1 (*) [16])(plVar11 + 2);
  plVar11[2] = 0;
  plVar11[3] = 0xf;
  *(undefined1 *)plVar11 = 0;
  uVar29 = 0xe00;
  pppppcVar21 = &local_148;
  if (0xf < uVar22) {
    pppppcVar21 = (char *****)local_148;
  }
  FUN_18001bfc0(&local_e8,pppppcVar21,lVar24);
  if (0xf < (ulonglong)local_138._8_8_) {
    pppppcVar21 = (char *****)local_148;
    if ((0xfff < local_138._8_8_ + 1) &&
       (pppppcVar21 = (char *****)local_148[-1],
       0x1f < (ulonglong)((longlong)local_148 + (-8 - (longlong)pppppcVar21)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pppppcVar21);
  }
  FUN_180018de0(&local_108);
  FUN_180018de0(local_68);
  if (0xf < (ulonglong)local_118._8_8_) {
    FUN_180019d40(&local_128,local_128);
  }
  local_118 = ZEXT816(0xf) << 0x40;
  local_128 = (undefined8 ****)((ulonglong)local_128 & 0xffffffffffffff00);
  if (0xf < uStack_150) {
    FUN_180019d40(&local_168,local_168);
  }
  local_190 = (double)CONCAT44(local_190._4_4_,0xffffffff);
  local_158 = 8;
  uStack_150 = 0xf;
  local_168 = (undefined8 ****)0x64695f726573616c;
  uStack_160 = 0;
  cVar5 = FUN_180055f90(param_1,&local_168);
  if (cVar5 == '\0') {
    if (*param_1 == 0) {
      iVar6 = (int)local_190;
    }
    else {
      iVar6 = FUN_180023c70(*param_1,&local_168,&local_190);
    }
  }
  else {
    iVar6 = FUN_180023b10(param_1,&local_168);
  }
  if (0xf < uStack_150) {
    ppppuVar14 = local_168;
    if ((0xfff < uStack_150 + 1) &&
       (ppppuVar14 = (undefined8 ****)local_168[-1],
       0x1f < (ulonglong)((longlong)local_168 + (-8 - (longlong)ppppuVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(ppppuVar14);
  }
  if (iVar6 != -1) {
    uVar10 = FUN_1800457c0(&local_168,iVar6);
    uVar10 = FUN_18001ffb0(local_68,&DAT_18028e198,uVar10);
    plVar11 = (longlong *)FUN_18001bfc0(uVar10,&DAT_18028e194,1);
    local_a8 = (undefined8 ****)*plVar11;
    pppcStack_a0 = (char ***)plVar11[1];
    lVar24 = *(longlong *)*(undefined1 (*) [16])(plVar11 + 2);
    uVar22 = plVar11[3];
    local_98 = *(undefined1 (*) [16])(plVar11 + 2);
    plVar11[2] = 0;
    plVar11[3] = 0xf;
    *(undefined1 *)plVar11 = 0;
    uVar29 = 0x1e00;
    pppppuVar26 = &local_a8;
    if (0xf < uVar22) {
      pppppuVar26 = (undefined8 *****)local_a8;
    }
    FUN_18001bfc0(&local_e8,pppppuVar26,lVar24);
    FUN_180018de0(&local_a8);
    FUN_180018de0(local_68);
    FUN_180018de0(&local_168);
  }
  uStack_160 = 0;
  local_158 = 4;
  uStack_150 = 0xf;
  local_168 = (undefined8 ****)0x656e6f6e;
  local_118 = _DAT_18028c630;
  local_128 = (undefined8 ****)&DAT_5f676e696b726f77;
  uStack_120 = 0x65646f6d5f7a;
  uVar30 = uVar29 | 0x100;
  cVar5 = FUN_180055f90(param_1,&local_128);
  if (cVar5 == '\0') {
    if (*param_1 == 0) {
      FUN_180017eb0(&local_88,&local_168);
    }
    else {
      FUN_1800243c0(*param_1,&local_88,&local_128,&local_168);
    }
  }
  else {
    FUN_180024440(param_1,&local_88,&local_128);
  }
  if (0xf < (ulonglong)local_118._8_8_) {
    ppppuVar14 = local_128;
    if ((0xfff < local_118._8_8_ + 1) &&
       (ppppuVar14 = (undefined8 ****)local_128[-1],
       (undefined1 *)0x1f < (undefined1 *)((longlong)local_128 + (-8 - (longlong)ppppuVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(ppppuVar14);
  }
  local_118 = ZEXT816(0xf) << 0x40;
  local_128 = (undefined8 ****)((ulonglong)local_128 & 0xffffffffffffff00);
  if (0xf < uStack_150) {
    ppppuVar14 = local_168;
    if ((0xfff < uStack_150 + 1) &&
       (ppppuVar14 = (undefined8 ****)local_168[-1],
       0x1f < (ulonglong)((longlong)local_168 + (-8 - (longlong)ppppuVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(ppppuVar14);
  }
  uVar22 = local_70;
  local_158 = 0;
  uStack_150 = 0xf;
  local_168 = (undefined8 ****)((ulonglong)local_168 & 0xffffffffffffff00);
  puVar2 = (undefined1 *)CONCAT71(uStack_87,local_88);
  puVar15 = &local_88;
  if (0xf < local_70) {
    puVar15 = puVar2;
  }
  if ((local_78 == 6) && (iVar7 = memcmp(puVar15,"manual",6), iVar7 == 0)) {
    FUN_18001bfc0(&local_e8,&DAT_18028efb0,4);
    local_190 = 0.0;
    local_f8 = (longlong *******)0x9;
    uStack_f0 = 0xf;
    local_108 = (longlong ********)&DAT_5f676e696b726f77;
    pppppppplStack_100 = (longlong ********)0x7a;
    cVar5 = FUN_180055f90(param_1,&local_108);
    if (cVar5 == '\0') {
      if (*param_1 != 0) {
        FUN_1800240d0(*param_1,&local_108,&local_190);
      }
    }
    else {
      FUN_180023f50(param_1,&local_108);
    }
    FUN_180029cd0(&local_e8);
    if (0xf < uStack_f0) {
      FUN_180019d40(&local_108,local_108);
    }
    FUN_18001bfc0(&local_e8,&DAT_18028efc4,2);
    local_138._8_8_ = 0xf;
    local_138._0_8_ = 0xc;
    local_148 = (char ****)0x735f676f6a5f7978;
    lStack_140 = 0x64656570;
    FUN_180023f50(param_1,&local_148);
    FUN_180029cd0(&local_e8);
    FUN_180018de0(&local_148);
    uVar10 = 1;
    pcVar20 = "\n";
LAB_1800354b5:
    FUN_18001bfc0(&local_e8,pcVar20,uVar10);
  }
  else {
    puVar15 = &local_88;
    if (0xf < uVar22) {
      puVar15 = puVar2;
    }
    if ((local_78 == 5) && (iVar7 = memcmp(puVar15,"lidar",5), iVar7 == 0)) {
      uVar10 = 9;
      pcVar20 = "LR_FOCUS\n";
      goto LAB_1800354b5;
    }
  }
  if (iVar6 == 1) {
    local_198[0] = '\0';
    local_138 = _DAT_18028fe90;
    local_148 = (char ****)0x616e655f61706f6d;
    lStack_140 = 0x64656c62;
    uVar30 = uVar29 | 0x101;
    local_190 = (double)(CONCAT44(local_190._4_4_,uVar29) | 0x101);
    cVar5 = FUN_180055f90(param_1,&local_148);
    if (cVar5 == '\0') {
      cVar5 = local_198[0];
      if (*param_1 != 0) {
        cVar5 = FUN_180025360(*param_1,&local_148,local_198);
      }
    }
    else {
      cVar5 = FUN_180025200(param_1,&local_148);
    }
    if (cVar5 == '\0') goto LAB_180035541;
    bVar4 = true;
  }
  else {
LAB_180035541:
    bVar4 = false;
  }
  if ((uVar30 & 1) != 0) {
    uVar30 = uVar30 & 0xfffffffe;
    FUN_180018de0(&local_148);
  }
  if (bVar4) {
    FUN_18001bfc0(&local_e8,"MOPA_SET_MO MO=1\n",0x11);
  }
  FUN_1800365d0(&local_108,param_1,1);
  cVar5 = local_198[0];
  local_180 = pppppppplStack_100;
  ppppppppplVar17 = (longlong *********)local_108;
  if (local_108 != pppppppplStack_100) {
    do {
      ppppppppplVar19 = ppppppppplVar17;
      if ((longlong ********)0xf < ppppppppplVar17[3]) {
        ppppppppplVar19 = (longlong *********)*ppppppppplVar17;
      }
      _Buf1 = ppppppppplVar19;
      if ((longlong ********)0xc < ppppppppplVar17[2]) {
        while( true ) {
          if ((*(char *)_Buf1 == 'W') && (iVar6 = memcmp(_Buf1,"WOBBLE_START ",0xd), iVar6 == 0)) {
            lVar24 = (longlong)_Buf1 - (longlong)ppppppppplVar19;
            goto LAB_1800355eb;
          }
          if (_Buf1 == ppppppppplVar19) break;
          _Buf1 = (longlong *********)((longlong)_Buf1 + -1);
        }
      }
      lVar24 = -1;
LAB_1800355eb:
      pppppppplVar28 = ppppppppplVar17[2];
      if (lVar24 == 0) {
        if (-(longlong)pppppppplVar28 + 0x7fffffffffffffffU < 5) {
                    /* WARNING: Subroutine does not return */
          FUN_18001ae90();
        }
        local_1b0 = (longlong ********)ppppppppplVar17;
        if ((longlong ********)0xf < ppppppppplVar17[3]) {
          local_1b0 = *ppppppppplVar17;
        }
        local_1b8 = (longlong *******)0x5;
        local_1a8 = (longlong *******)pppppppplVar28;
        FUN_180018010(&local_148,cVar5,ppppppppplVar17,"M400\n");
        uVar10 = local_138._0_8_;
        if ((ulonglong)(local_138._8_8_ - local_138._0_8_) < 10) {
          local_1b8 = (longlong *******)0xa;
          pppppcVar21 = (char *****)FUN_180016ae0(&local_148,10,0,"\nG00 P100\n");
        }
        else {
          local_138._0_8_ = local_138._0_8_ + 10;
          pppppcVar21 = &local_148;
          if (0xf < (ulonglong)local_138._8_8_) {
            pppppcVar21 = (char *****)local_148;
          }
          memmove((void *)((longlong)pppppcVar21 + uVar10),"\nG00 P100\n",10);
          *(undefined1 *)((longlong)pppppcVar21 + uVar10 + 10) = 0;
          pppppcVar21 = &local_148;
        }
        pppppuVar26 = (undefined8 *****)*pppppcVar21;
        local_a8 = pppppuVar26;
        pppcStack_a0 = (char ***)pppppcVar21[1];
        auVar1 = *(undefined1 (*) [16])(pppppcVar21 + 2);
        local_98 = auVar1;
        pppppcVar21[2] = (char ****)0x0;
        pppppcVar21[3] = (char ****)0xf;
        *(undefined1 *)pppppcVar21 = 0;
        uVar30 = uVar30 | 0x18;
        local_190 = auVar1._8_8_;
        pppppuVar27 = &local_a8;
        if (0xf < (ulonglong)local_190) {
          pppppuVar27 = pppppuVar26;
        }
        local_188 = local_d8;
        pppppppplVar28 = auVar1._0_8_;
        if ((longlong ********)(uStack_d0 - (longlong)local_d8) < pppppppplVar28) {
          local_1b8 = (longlong *******)pppppppplVar28;
          FUN_180016ae0(&local_e8,pppppppplVar28,0);
          dVar31 = (double)local_98._8_8_;
          pppppuVar26 = (undefined8 *****)local_a8;
        }
        else {
          pppppuVar23 = &local_e8;
          if (0xf < uStack_d0) {
            pppppuVar23 = (undefined8 *****)local_e8;
          }
          pvVar16 = (void *)((longlong)local_d8 + (longlong)pppppuVar23);
          local_d8 = (double)((longlong)local_d8 + (longlong)pppppppplVar28);
          memmove(pvVar16,pppppuVar27,(size_t)pppppppplVar28);
          *(undefined1 *)((longlong)local_188 + (longlong)pppppuVar23 + (longlong)pppppppplVar28) =
               0;
          dVar31 = local_190;
        }
        if (0xf < (ulonglong)dVar31) {
          pppppuVar27 = pppppuVar26;
          if ((0xfff < (longlong)dVar31 + 1U) &&
             (pppppuVar27 = (undefined8 *****)pppppuVar26[-1],
             0x1f < (ulonglong)((longlong)pppppuVar26 + (-8 - (longlong)pppppuVar27)))) {
                    /* WARNING: Subroutine does not return */
            _invalid_parameter_noinfo_noreturn();
          }
          free(pppppuVar27);
        }
        if (0xf < (ulonglong)local_138._8_8_) {
          FUN_180019d40(&local_148,local_148);
        }
      }
      else {
        if (-(longlong)pppppppplVar28 == -0x7fffffffffffffff) {
                    /* WARNING: Subroutine does not return */
          FUN_18001ae90();
        }
        ppppppppplVar19 = ppppppppplVar17;
        if ((longlong ********)0xf < ppppppppplVar17[3]) {
          ppppppppplVar19 = (longlong *********)*ppppppppplVar17;
        }
        local_1a8 = (longlong *******)0x1;
        local_1b0 = (longlong ********)&DAT_18028e194;
        local_1b8 = (longlong *******)pppppppplVar28;
        FUN_180018010(&local_148,cVar5,ppppppppplVar17,ppppppppplVar19);
        pppppcVar21 = (char *****)local_148;
        uVar30 = uVar30 | 0x40;
        pppppcVar18 = &local_148;
        if (0xf < (ulonglong)local_138._8_8_) {
          pppppcVar18 = (char *****)local_148;
        }
        uVar10 = local_138._0_8_;
        if ((longlong ********)(uStack_d0 - (longlong)local_d8) < (ulonglong)local_138._0_8_) {
          local_1b8 = (longlong *******)local_138._0_8_;
          FUN_180016ae0(&local_e8,local_138._0_8_,0);
          pppppcVar21 = (char *****)local_148;
        }
        else {
          pppppuVar26 = &local_e8;
          if (0xf < uStack_d0) {
            pppppuVar26 = (undefined8 *****)local_e8;
          }
          pvVar16 = (void *)((longlong)pppppuVar26 + (longlong)local_d8);
          local_d8 = (double)((longlong)local_d8 + local_138._0_8_);
          memmove(pvVar16,pppppcVar18,local_138._0_8_);
          *(undefined1 *)((longlong)pvVar16 + uVar10) = 0;
        }
        if (0xf < (ulonglong)local_138._8_8_) {
          pppppcVar18 = pppppcVar21;
          if ((0xfff < local_138._8_8_ + 1) &&
             (pppppcVar18 = (char *****)pppppcVar21[-1],
             0x1f < (ulonglong)((longlong)pppppcVar21 + (-8 - (longlong)pppppcVar18)))) {
                    /* WARNING: Subroutine does not return */
            _invalid_parameter_noinfo_noreturn();
          }
          free(pppppcVar18);
        }
      }
      ppppppppplVar17 = ppppppppplVar17 + 4;
      param_1 = local_178[0];
    } while (ppppppppplVar17 != (longlong *********)local_180);
  }
  if ((longlong *********)local_108 != (longlong *********)0x0) {
    FUN_180015de0(local_108,pppppppplStack_100,&local_108);
    ppppppppplVar17 = (longlong *********)local_108;
    if ((0xfff < ((longlong)local_f8 - (longlong)local_108 & 0xffffffffffffffe0U)) &&
       (ppppppppplVar17 = (longlong *********)local_108[-1],
       (char *)0x1f < (char *)((longlong)local_108 + (-8 - (longlong)ppppppppplVar17)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(ppppppppplVar17);
  }
  local_c8 = (char ****)0x0;
  uStack_c0 = 0;
  pppppcVar21 = (char *****)0x0;
  local_b8 = 0;
  local_b0 = 0;
  local_c8 = (char ****)FUN_18024833c(0x20);
  local_b8 = 0x15;
  local_b0 = 0x1f;
  *local_c8 = (char ***)0x5f5245444f434e45;
  local_c8[1] = (char ***)0x414e45204e525554;
  *(undefined4 *)(local_c8 + 2) = 0x3d454c42;
  *(char *)((longlong)local_c8 + 0x14) = '0';
  *(char *)((longlong)local_c8 + 0x15) = '\0';
  cVar5 = FUN_18004c9f0(param_1);
  uVar22 = local_b0;
  if (cVar5 != '\0') {
    if (local_b0 < 0x1f) {
      if (0x7fffffffffffffff - (local_b0 >> 1) < local_b0) {
        uVar25 = 0x7fffffffffffffff;
        uVar13 = 0x8000000000000027;
LAB_1800359ee:
        ppppcVar12 = (char ****)FUN_18024833c(uVar13);
        if (ppppcVar12 == (char ****)0x0) goto LAB_180035ddb;
        pppppcVar21 = (char *****)((longlong)ppppcVar12 + 0x27U & 0xffffffffffffffe0);
        pppppcVar21[-1] = ppppcVar12;
      }
      else {
        uVar13 = (local_b0 >> 1) + local_b0;
        uVar25 = 0x1f;
        if (0x1f < uVar13) {
          uVar25 = uVar13;
        }
        if (0xfff < uVar25 + 1) {
          uVar13 = uVar25 + 0x28;
          if (uVar13 <= uVar25 + 1) {
                    /* WARNING: Subroutine does not return */
            FUN_18001a930();
          }
          goto LAB_1800359ee;
        }
        if (uVar25 != 0xffffffffffffffff) {
          pppppcVar21 = (char *****)FUN_18024833c();
        }
      }
      local_b8 = 0x1f;
      *pppppcVar21 = (char ****)0x5f5245444f434e45;
      pppppcVar21[1] = (char ****)0x414e45204e525554;
      pppppcVar21[2] = (char ****)0x535520313d454c42;
      *(undefined4 *)(pppppcVar21 + 3) = 0x414d5f45;
      ((char *)((longlong)pppppcVar21 + 0x1c))[0] = 'P';
      ((char *)((longlong)pppppcVar21 + 0x1c))[1] = '=';
      *(char *)((longlong)pppppcVar21 + 0x1e) = '1';
      *(char *)((longlong)pppppcVar21 + 0x1f) = '\0';
      local_b0 = uVar25;
      if (0xf < uVar22) {
        pppppcVar18 = (char *****)local_c8;
        if ((0xfff < uVar22 + 1) &&
           (pppppcVar18 = (char *****)local_c8[-1],
           (char *)0x1f < (char *)((longlong)local_c8 + (-8 - (longlong)pppppcVar18))))
        goto LAB_180035ddb;
        free(pppppcVar18);
      }
    }
    else {
      pppppcVar21 = &local_c8;
      if (0xf < local_b0) {
        pppppcVar21 = (char *****)local_c8;
      }
      local_b8 = 0x1f;
      memmove(pppppcVar21,"ENCODER_TURN ENABLE=1 USE_MAP=1",0x1f);
      *(char *)((longlong)pppppcVar21 + 0x1f) = '\0';
      pppppcVar21 = (char *****)local_c8;
    }
    local_c8 = (char ****)pppppcVar21;
    uStack_120 = 0;
    local_118 = ZEXT816(0xf) << 0x40;
    local_128 = (undefined8 ****)0x0;
    local_148 = (char ****)0x0;
    lStack_140 = 0;
    local_138 = ZEXT816(0);
    local_148 = (char ****)FUN_18024833c(0x20);
    local_138._8_8_ = 0x1f;
    local_138._0_8_ = 0x11;
    *local_148 = (char ***)0x5f7265646f636e65;
    local_148[1] = (char ***)0x6978615f6e727574;
    *(char *)(local_148 + 2) = 's';
    *(char *)((longlong)local_148 + 0x11) = '\0';
    cVar5 = FUN_180055f90(param_1,&local_148);
    if (cVar5 == '\0') {
      if (*param_1 == 0) {
        FUN_180017eb0(&local_108,&local_128);
      }
      else {
        FUN_1800243c0(*param_1,&local_108,&local_148,&local_128);
      }
    }
    else {
      FUN_180024440(param_1,&local_108,&local_148);
    }
    if (0xf < (ulonglong)local_138._8_8_) {
      ppppcVar12 = local_148;
      if ((0xfff < local_138._8_8_ + 1) &&
         (ppppcVar12 = (char ****)local_148[-1],
         (char *)0x1f < (char *)((longlong)local_148 + (-8 - (longlong)ppppcVar12)))) {
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(ppppcVar12);
    }
    local_138 = ZEXT816(0xf) << 0x40;
    local_148 = (char ****)((ulonglong)local_148 & 0xffffffffffffff00);
    if (0xf < (ulonglong)local_118._8_8_) {
      ppppuVar14 = local_128;
      if ((0xfff < local_118._8_8_ + 1) &&
         (ppppuVar14 = (undefined8 ****)local_128[-1],
         0x1f < (ulonglong)((longlong)local_128 + (-8 - (longlong)ppppuVar14)))) {
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(ppppuVar14);
    }
    local_118 = ZEXT816(0xf) << 0x40;
    local_128 = (undefined8 ****)((ulonglong)local_128 & 0xffffffffffffff00);
    local_158 = 0xf;
    uStack_150 = 0xf;
    local_168 = (undefined8 *****)0x676e696e6e616c70;
    uStack_160 = 0x646f6874656d5f;
    local_190 = (double)(CONCAT44(local_190._4_4_,uVar30) | 0x22);
    iVar6 = FUN_180024d10(param_1,&local_168);
    uVar22 = uStack_f0;
    pppppppplVar28 = (longlong ********)local_f8;
    ppppppppplVar17 = (longlong *********)local_108;
    if (iVar6 == 10) {
      ppppppppplVar19 = &local_108;
      if (0xf < uStack_f0) {
        ppppppppplVar19 = (longlong *********)local_108;
      }
      if (((longlong ********)local_f8 != (longlong ********)0x1) ||
         (iVar6 = memcmp(ppppppppplVar19,&DAT_18028e1a0,1), iVar6 != 0)) {
        ppppppppplVar19 = &local_108;
        if (0xf < uVar22) {
          ppppppppplVar19 = ppppppppplVar17;
        }
        if ((pppppppplVar28 != (longlong ********)0x1) ||
           (iVar6 = memcmp(ppppppppplVar19,&DAT_18028e19c,1), iVar6 != 0)) goto LAB_180035cca;
      }
      bVar4 = true;
    }
    else {
LAB_180035cca:
      bVar4 = false;
    }
    if (0xf < uStack_150) {
      pppppuVar26 = (undefined8 *****)local_168;
      if ((0xfff < uStack_150 + 1) &&
         (pppppuVar26 = (undefined8 *****)local_168[-1],
         0x1f < (ulonglong)((longlong)local_168 + (-8 - (longlong)pppppuVar26)))) {
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(pppppuVar26);
      ppppppppplVar17 = (longlong *********)local_108;
      uVar22 = uStack_f0;
      pppppppplVar28 = (longlong ********)local_f8;
    }
    if (bVar4) {
      if (0x7fffffffffffffffU - (longlong)pppppppplVar28 < 6) {
                    /* WARNING: Subroutine does not return */
        FUN_18001ae90();
      }
      local_1b0 = (longlong ********)&local_108;
      if (0xf < uVar22) {
        local_1b0 = (longlong ********)ppppppppplVar17;
      }
      local_1b8 = (longlong *******)0x6;
      local_1a8 = (longlong *******)pppppppplVar28;
      FUN_180018010(&local_168,local_198[0],&local_108," AXIS=");
      pppppuVar26 = &local_168;
      if (0xf < uStack_150) {
        pppppuVar26 = (undefined8 *****)local_168;
      }
      FUN_18001bfc0(&local_c8,pppppuVar26,local_158);
      ppppppppplVar17 = (longlong *********)local_108;
      uVar22 = uStack_f0;
      if (0xf < uStack_150) {
        FUN_180019d40(&local_168,local_168);
        ppppppppplVar17 = (longlong *********)local_108;
        uVar22 = uStack_f0;
      }
    }
    if (0xf < uVar22) {
      ppppppppplVar19 = ppppppppplVar17;
      if ((0xfff < uVar22 + 1) &&
         (ppppppppplVar19 = (longlong *********)ppppppppplVar17[-1],
         (char *)0x1f < (char *)((longlong)ppppppppplVar17 + (-8 - (longlong)ppppppppplVar19)))) {
LAB_180035ddb:
                    /* WARNING: Subroutine does not return */
        _invalid_parameter_noinfo_noreturn();
      }
      free(ppppppppplVar19);
    }
  }
  _Size = local_b8;
  if (local_b8 == 0x7fffffffffffffff) {
                    /* WARNING: Subroutine does not return */
    FUN_18001ae90();
  }
  pppppcVar21 = &local_c8;
  if (0xf < local_b0) {
    pppppcVar21 = (char *****)local_c8;
  }
  local_128 = (undefined8 *****)0x0;
  uStack_120 = 0;
  local_118._0_8_ = 0;
  uVar10 = local_118._0_8_;
  local_118._0_8_ = (longlong ********)0x0;
  local_118._8_8_ = 0;
  pppppppplVar28 = (longlong ********)(local_b8 + 1);
  uVar22 = 0xf;
  pppppuVar26 = &local_128;
  if ((longlong ********)0xf < pppppppplVar28) {
    uVar22 = (ulonglong)pppppppplVar28 | 0xf;
    if (uVar22 < 0x8000000000000000) {
      if (uVar22 < 0x16) {
        uVar22 = 0x16;
      }
      if (uVar22 + 1 < 0x1000) {
        if (uVar22 == 0xffffffffffffffff) {
          pppppuVar26 = (undefined8 *****)0x0;
          local_128 = pppppuVar26;
        }
        else {
          pppppuVar26 = (undefined8 *****)FUN_18024833c();
          local_128 = pppppuVar26;
        }
        goto LAB_180035ea6;
      }
      uVar13 = uVar22 + 0x28;
      if (uVar13 <= uVar22 + 1) {
        local_118._0_8_ = uVar10;
                    /* WARNING: Subroutine does not return */
        FUN_18001a930();
      }
    }
    else {
      uVar22 = 0x7fffffffffffffff;
      uVar13 = 0x8000000000000027;
    }
    ppppuVar14 = (undefined8 ****)FUN_18024833c(uVar13);
    if (ppppuVar14 == (undefined8 ****)0x0) goto LAB_180035f67;
    pppppuVar26 = (undefined8 *****)((longlong)ppppuVar14 + 0x27U & 0xffffffffffffffe0);
    pppppuVar26[-1] = ppppuVar14;
    local_128 = pppppuVar26;
  }
LAB_180035ea6:
  local_118._0_8_ = pppppppplVar28;
  local_118._8_8_ = uVar22;
  memcpy(pppppuVar26,pppppcVar21,_Size);
  *(undefined1 *)((longlong)pppppuVar26 + _Size) = 10;
  *(undefined1 *)((longlong)pppppuVar26 + (longlong)pppppppplVar28) = 0;
  dVar31 = local_d8;
  uVar22 = local_118._8_8_;
  uVar10 = local_118._0_8_;
  pppppuVar26 = (undefined8 *****)local_128;
  pppppuVar27 = &local_128;
  if (0xf < (ulonglong)local_118._8_8_) {
    pppppuVar27 = (undefined8 *****)local_128;
  }
  if ((longlong ********)(uStack_d0 - (longlong)local_d8) < (ulonglong)local_118._0_8_) {
    local_1b8 = (longlong *******)local_118._0_8_;
    FUN_180016ae0(&local_e8,local_118._0_8_,0);
    pppppuVar26 = (undefined8 *****)local_128;
    uVar22 = local_118._8_8_;
  }
  else {
    pppppuVar23 = &local_e8;
    if (0xf < uStack_d0) {
      pppppuVar23 = (undefined8 *****)local_e8;
    }
    pvVar16 = (void *)((longlong)pppppuVar23 + (longlong)local_d8);
    local_d8 = (double)((longlong)local_d8 + local_118._0_8_);
    memmove(pvVar16,pppppuVar27,local_118._0_8_);
    *(undefined1 *)(uVar10 + (longlong)pppppuVar23 + (longlong)dVar31) = 0;
  }
  if (0xf < uVar22) {
    pppppuVar27 = pppppuVar26;
    if ((0xfff < uVar22 + 1) &&
       (pppppuVar27 = (undefined8 *****)pppppuVar26[-1],
       0x1f < (ulonglong)((longlong)pppppuVar26 + (-8 - (longlong)pppppuVar27)))) {
LAB_180035f67:
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pppppuVar27);
  }
  local_158 = 0xb;
  uStack_150 = 0xf;
  local_168 = (undefined8 ****)0x63675f7472617473;
  uStack_160 = 0x65646f;
  uVar10 = FUN_1800536b0(param_1,&local_a8,&local_e8);
  plVar11 = (longlong *)FUN_180017120(param_1 + 2,local_178,&local_168);
  FUN_180019770(*plVar11 + 0x30,uVar10);
  if (0xf < (ulonglong)local_98._8_8_) {
    pppppuVar26 = (undefined8 *****)local_a8;
    if ((0xfff < local_98._8_8_ + 1) &&
       (pppppuVar26 = (undefined8 *****)local_a8[-1],
       0x1f < (ulonglong)((longlong)local_a8 + (-8 - (longlong)pppppuVar26)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pppppuVar26);
  }
  if (0xf < uStack_150) {
    ppppuVar14 = local_168;
    if ((0xfff < uStack_150 + 1) &&
       (ppppuVar14 = (undefined8 ****)local_168[-1],
       0x1f < (ulonglong)((longlong)local_168 + (-8 - (longlong)ppppuVar14)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(ppppuVar14);
  }
  if (0xf < local_b0) {
    pppppcVar21 = (char *****)local_c8;
    if ((0xfff < local_b0 + 1) &&
       (pppppcVar21 = (char *****)local_c8[-1],
       (char *)0x1f < (char *)((longlong)local_c8 + (-8 - (longlong)pppppcVar21)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pppppcVar21);
  }
  local_b8 = 0;
  local_b0 = 0xf;
  local_c8 = (char ****)((ulonglong)local_c8 & 0xffffffffffffff00);
  if (0xf < local_70) {
    pvVar3 = (void *)CONCAT71(uStack_87,local_88);
    pvVar16 = pvVar3;
    if ((0xfff < local_70 + 1) &&
       (pvVar16 = *(void **)((longlong)pvVar3 + -8),
       0x1f < (ulonglong)((longlong)pvVar3 + (-8 - (longlong)pvVar16)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pvVar16);
  }
  local_78 = 0;
  local_70 = 0xf;
  local_88 = 0;
  if (0xf < uStack_d0) {
    pppppuVar26 = (undefined8 *****)local_e8;
    if ((0xfff < uStack_d0 + 1) &&
       (pppppuVar26 = (undefined8 *****)local_e8[-1],
       0x1f < (ulonglong)((longlong)local_e8 + (-8 - (longlong)pppppuVar26)))) {
                    /* WARNING: Subroutine does not return */
      _invalid_parameter_noinfo_noreturn();
    }
    free(pppppuVar26);
  }
  return;
}



## confidence

observed

## authority

shipped-artifact

## environment

null

## limitations

The session serves operations only after Ghidra reports default auto-analysis complete; incomplete analysis does not expose partial results.,The imported Program and temporary project are ephemeral and deleted on close. Annotation operations edit session database metadata; original executable bytes are never written.,Cancelling an active Ghidra request terminates its ephemeral database and discards session annotations. The selected target remains open; the next provider query imports the original artifact again.,Default-space addresses use lowercase 0x-prefixed hexadecimal; other address spaces use <percent-encoded-space>:0x<hex>.,Pseudocode is Ghidra decompiler output, not original source and not text-equivalent to Hopper output.,External functions and functions without an analyzable body return null; other decompiler failures remain explicit.,Ghidra Listing reports a terminal call at 0x180035023 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003512a to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x1800352bd to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003530d to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180035918 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003591f to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180035926 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180035b9f to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180035bee to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180035d06 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180035ddb to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180035f67 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003601d to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003605d to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003609c to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x1800360eb to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036139 to EXTERNAL:0xcd API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL::_invalid_parameter_noinfo_noreturn (external=true, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003617a to 0x18001ae90 FUN_18001ae90 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036180 to 0x18001a930 FUN_18001a930 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036186 to 0x18001a930 FUN_18001a930 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003618c to 0x18001ae90 FUN_18001ae90 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036192 to 0x18001ae90 FUN_18001ae90 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x180036198 to 0x18001ae90 FUN_18001ae90 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Ghidra Listing reports a terminal call at 0x18003619e to 0x18001ae90 FUN_18001ae90 (external=false, thunk=false, hasNoReturn=true); fallthrough is excluded by Ghidra's flow model. This records Ghidra's FunctionManager flag, not an independent verification that the callee cannot return.,Windows Ghidra P0 accepts native x86, x86-64, ARMNT (Thumb-2) and ARM64 PE applications and DLLs, and explicitly selected raw 6502 images; managed, hostile, sensitive, and mutable-path targets are unsupported.,The Windows bridge uses authenticated IPv4 loopback because Node path-based IPC does not expose Java AF_UNIX sockets; the endpoint file contains no bearer token.,Windows sessions require the matching packaged Windows x64 native addon, local NTFS targets and runtimes, and Windows 10 or later. Native handles enforce path admission, private DACLs, and Job Object ownership automatically.

## relationship_coverage

null

## locations



## evidence_links

