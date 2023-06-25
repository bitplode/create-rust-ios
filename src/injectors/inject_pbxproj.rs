use std::{path::PathBuf, str::FromStr};
use crate::configuration::Config;

struct Section {
  pub name: String,
  pub content: String
}

impl Section {
  pub fn set_content(&mut self, content: &str) {
    self.content = content.to_string();
  }
}

type SectionCollection = Vec<Section>;

fn get_sections(content: &str) -> SectionCollection {
  let mut sections = SectionCollection::new();

  let mut old_i: usize = 0;
  loop {
    let i_ret = content.chars().skip(old_i).collect::<String>().find("/* Begin").map(|x| x + old_i);
    if i_ret.is_none() {
      break;
    }

    let i = i_ret.unwrap();

    let j_ret = content.chars().skip(i).collect::<String>().find("*/").map(|x| x + i);
    if j_ret.is_none() {
      break;
    }

    let j = j_ret.unwrap();

    let begin_label = content.chars().skip(i + 3).take(j - 4 - i).collect::<String>();
    let raw_label = begin_label.replace("Begin ", "");
    let end_label = format!("/* End {raw_label} */");

    let end_i_ret = content.find(&end_label);
    if end_i_ret.is_none() {
      break;
    }
    let end_i = end_i_ret.unwrap();

    let raw_content = content.chars().skip(j + 2).take(end_i - j - 2).collect::<String>();
    sections.push(Section { name: raw_label, content: raw_content });

    old_i = end_i;
  }

  sections
}

const LIB_FILE_ID: &str = "F9A8DB0E2A3F60CC00A5F46A";
const LIB_FILE_REF: &str = "F9A8DB082A39639800A5F46A";

const BRIDGE_HEADER_ID: &str = "F9A8DB0B2A3D489100A5F46A";
const NATIVE_TARGET_FRAMEWORKS_ID: &str = "F96881FC2A28405700C2DF3B";

const DEFAULT_FRAMEWORKS_ID: &str = "F9A8DB072A39639700A5F46A";

fn handle_pbx_build_file_section(config: &Config, sections: &mut SectionCollection) {
  if let Some(section) = sections.iter_mut().find(|x| x.name == "PBXBuildFile section") {
    let lib_name = config.get_ios_lib_executable_name();
    if !section.content.contains(&lib_name) {
      let line = format!("		{LIB_FILE_ID} /* {lib_name} in Frameworks */ = {{isa = PBXBuildFile; fileRef = {LIB_FILE_REF} /* {lib_name} */; }};");
      section.set_content(&format!("{}{line}\n", section.content));
    }
  }
}

fn handle_pbx_file_reference_section(config: &Config, sections: &mut SectionCollection) {
  if let Some(section) = sections.iter_mut().find(|x| x.name == "PBXFileReference section") {
    let lib_name = config.get_ios_lib_executable_name();
    let bridge_header_name = config.get_bridge_h_file_name();
    if !section.content.contains(&lib_name) {
      let line1 = format!("		{LIB_FILE_REF} /* {lib_name} */ = {{isa = PBXFileReference; lastKnownFileType = archive.ar; name = {lib_name}; path = libs/{lib_name}; sourceTree = \"<group>\"; }};");
      let line2 = format!("		{BRIDGE_HEADER_ID} /* {bridge_header_name} */ = {{isa = PBXFileReference; lastKnownFileType = sourcecode.c.h; path = \"{bridge_header_name}\"; sourceTree = \"<group>\"; }};");
      section.set_content(&format!("{}{line1}\n{line2}\n", section.content));
    }
  }
}

fn get_frameworks_id_from_pbx_native_target_section(sections: &SectionCollection) -> Result<String, String> {
  if let Some(section) = sections.iter().find(|x| x.name == "PBXNativeTarget section") {
    let ff_id_ret = get_pbx_group_children_id(&section.content, " /* Frameworks */,");
    if ff_id_ret.is_none() {
      return Err("Cannot find Frameworks section in PBXNativeTarget section".to_string());
    }
    return Ok(ff_id_ret.unwrap());
  }

  Err("Cannot find the PBXNativeTarget section".to_string())
}

fn handle_pbx_frameworks_build_phase_section(config: &Config, sections: &mut SectionCollection) {
  let frameworks_id_ret = get_frameworks_id_from_pbx_native_target_section(sections);
  if let Err(err) = frameworks_id_ret {
    panic!("{err}");
  }
  let frameworks_id = frameworks_id_ret.unwrap();

  if let Some(section) = sections.iter_mut().find(|x| x.name == "PBXFrameworksBuildPhase section") {
    let lib_name = config.get_ios_lib_executable_name();
    if !section.content.contains(&lib_name) {
      // TODO: Change NATIVE_TARGET_FRAMEWORKS_ID to be the value taken from
      // F96881FC2A28405700C2DF3B /* Frameworks */, in the following section!!!!!
      /*
/* Begin PBXNativeTarget section */
		F96881FE2A28405700C2DF3B /* TestUI */ = {
			isa = PBXNativeTarget;
			buildConfigurationList = F96882232A28405A00C2DF3B /* Build configuration list for PBXNativeTarget "TestUI" */;
			buildPhases = (
				F96881FB2A28405700C2DF3B /* Sources */,
				F96881FC2A28405700C2DF3B /* Frameworks */,
				F96881FD2A28405700C2DF3B /* Resources */,
			);
			buildRules = (
			);
			dependencies = (
			);
			name = TestUI;
			productName = TestUI;
			productReference = F96881FF2A28405700C2DF3B /* TestUI.app */;
			productType = "com.apple.product-type.application";
		};

       */
      let mut line = format!("		{frameworks_id} /* Frameworks */ = {{\n");
      line.push_str("			isa = PBXFrameworksBuildPhase;\n");
      line.push_str("			buildActionMask = 2147483647;\n");
      line.push_str("			files = (\n");
      line.push_str(&format!("				{LIB_FILE_ID} /* {lib_name} in Frameworks */,\n"));
      line.push_str("			);\n");
      line.push_str("			runOnlyForDeploymentPostprocessing = 0;\n");
      line.push_str("		};\n");
      section.set_content(&format!("{}{line}\n", section.content));
    }
  }
}

fn get_pbx_group_children_id(section_content: &str, identifier: &str) -> Option<String> {
  let loc_frameworks_comment = section_content.find(identifier)?;
  let (before, _) = section_content.split_at(loc_frameworks_comment);
  let loc_prev_line = before.rfind('\n')?;
  let (_, frameworks_id) = before.split_at(loc_prev_line);
  Some(frameworks_id.trim().to_string())
}

fn handle_pbx_group_section(config: &Config, sections: &mut SectionCollection) {
  if let Some(section) = sections.iter_mut().find(|x| x.name == "PBXGroup section") {
    let lib_name = config.get_ios_lib_executable_name();
    if !section.content.contains(&lib_name) {
      let mut ff_id_ret = get_pbx_group_children_id(&section.content, " /* Frameworks */,");
      if ff_id_ret.is_none() {
        println!(">>> DID NOT FIND!!!!!!!!!!");
        let loc_products_ret = section.content.find(" /* Products */,");
        if loc_products_ret.is_none() {
          return;
        }
        let loc_products = loc_products_ret.unwrap();
        let (pre, post) = section.content.split_at(loc_products + 16);
        section.content = format!("{pre}\n				{DEFAULT_FRAMEWORKS_ID} /* Frameworks */,{post}");
        ff_id_ret = Some(DEFAULT_FRAMEWORKS_ID.to_string());
      }
      let frameworks_id = ff_id_ret.unwrap();

      // now write the frameworks section for lib_name
      let mut line = format!("		{} /* Frameworks */ = {{\n", frameworks_id.trim());
      line.push_str("			isa = PBXGroup;\n");
      line.push_str("			children = (\n");
      line.push_str(&format!("				{LIB_FILE_REF} /* {lib_name} */,\n"));
      line.push_str("			);\n");
      line.push_str("			name = Frameworks;\n");
      line.push_str("			sourceTree = \"<group>\";\n");
      line.push_str("		};\n");
      section.set_content(&format!("{}{line}\n", section.content));
    }

    let bridge_header = config.get_bridge_h_file_name();
    if !section.content.contains(&bridge_header) {
      // get the ios_proj_id
      let ios_dir_name = config.get_ios_dir_name();

      let loc_proj_comment_ret = section.content.find(&format!(" /* {ios_dir_name} */ = {{"));
      if loc_proj_comment_ret.is_none() {
        return;
      }
      let loc_proj_comment = loc_proj_comment_ret.unwrap();

      let loc_children_ret = section.content.chars().skip(loc_proj_comment).collect::<String>().find("children = (").map(|x| x + loc_proj_comment);
      if loc_children_ret.is_none() {
        return;
      }
      let loc_children = loc_children_ret.unwrap();

      let (pre, post) = section.content.split_at(loc_children + 12);

      let line = format!("				{BRIDGE_HEADER_ID} /* {bridge_header} */,");

      section.set_content(&format!("{pre}\n{line}{post}"))
    }
  }
}

fn get_build_settings(section_content: &str, index: usize) -> Option<(String, usize, usize)> {
  let loc1_ret = section_content.chars().take(index).collect::<String>().rfind("buildSettings = {");
  let loc2_ret = section_content.chars().skip(index).collect::<String>().find("};").map(|x| x + index);
  if loc1_ret.is_none() || loc2_ret.is_none() {
    return None;
  }

  let loc1 = loc1_ret.unwrap();
  let loc2 = loc2_ret.unwrap();
  let piece = section_content.chars().skip(loc1).take(loc2 - loc1).collect::<String>();

  Some((piece, loc1, loc2))
}

fn modify_build_settings(config: &Config, build_settings: &str) -> String {
  let mut result = build_settings.to_string();
  let libs_path = config.get_ios_libs_directory_path();
  let ios_dir_name = config.get_ios_dir_name();
  let bridge_header_name = config.get_bridge_h_file_name();
  let line1 = format!("			LIBRARY_SEARCH_PATHS = \"{libs_path}\";\n");
  let line2 = format!("				\"LIBRARY_SEARCH_PATHS[arch=*]\" = \"{libs_path}\";\n");
  let line3 = format!("				SWIFT_OBJC_BRIDGING_HEADER = \"{ios_dir_name}/{bridge_header_name}\";\n			");

  if !result.contains(&line1) {
    result.push_str(&line1);
  }
  if !result.contains(&line2) {
    result.push_str(&line2);
  }
  if !result.contains(&line3) {
    result.push_str(&line3);
  }
  result
}

fn process_build_settings(config: &Config, section: &mut Section, index: usize) -> Result<(), String> {
  let build_settings_ret = get_build_settings(&section.content, index);
  if build_settings_ret.is_none() {
    return Err(format!("Cannot find the buildSettings section near location {index}"));
  }
  let build_settings = build_settings_ret.unwrap();
  let modified_bs = modify_build_settings(config, &build_settings.0);
  let pre = section.content.chars().take(build_settings.1).collect::<String>();
  let post = section.content.chars().skip(build_settings.2).collect::<String>();
  section.content = format!("{pre}{modified_bs}{post}");
  Ok(())
}

fn handle_lib_search(config: &Config, sections: &mut SectionCollection) -> Result<(), String> {
  if let Some(section) = sections.iter_mut().find(|x| x.name == "XCBuildConfiguration section") {
    let loc1_ret = section.content.find("LD_RUNPATH_SEARCH_PATHS");
    if loc1_ret.is_none() {
      return Err("Cannot find the first LD_RUNPATH_SEARCH_PATHS".to_string());
    }
    let loc1 = loc1_ret.unwrap();

    let loc2_ret = section.content.rfind("LD_RUNPATH_SEARCH_PATHS");
    if loc2_ret.is_none() {
      return Err("Cannot find the second LD_RUNPATH_SEARCH_PATHS".to_string());
    }
    let loc2 = loc2_ret.unwrap();

    if loc1 == loc2 {
      return Err("We only found 1 LD_RUNPATH_SEARCH_PATHS".to_string());
    }

    process_build_settings(config, section, loc1)?;
    process_build_settings(config, section, loc2)?;
  }

  Ok(())
}

fn print_sections(sections: &SectionCollection) -> String {
  let mut result = String::from("");
  for section in sections {
    result.push_str(&format!("/* Begin {} */\n", section.name));
    result.push_str(&section.content);
    result.push_str(&format!("/* End {} */\n", section.name));
  }
  result
}

fn parse(config: &Config, content: String) -> Result<String, String> {
  let mut sections = get_sections(&content);
  handle_pbx_build_file_section(config, &mut sections);
  handle_pbx_file_reference_section(config, &mut sections);
  handle_pbx_frameworks_build_phase_section(config, &mut sections);
  handle_pbx_group_section(config, &mut sections);
  handle_lib_search(config, &mut sections)?;
  // println!(">>> EDITED | sections = {sections:#?}");

  let loc_preamble_ret = content.find("/* Begin");
  if loc_preamble_ret.is_none() {
    return Err("Cannot find initial section of pbxproj file".to_string());
  }
  let (preamble_content, _) = content.split_at(loc_preamble_ret.unwrap());

  const SUFFIX_SUB_STRING: &str = " section */";
  let loc_suffix_ret = content.rfind(SUFFIX_SUB_STRING);
  if loc_suffix_ret.is_none() {
    return Err("Cannot find trailing section of pbxproj file".to_string());
  }
  let (_, suffix_content) = content.split_at(loc_suffix_ret.unwrap() + SUFFIX_SUB_STRING.len());

  let final_content = format!("{preamble_content}{}{suffix_content}", print_sections(&sections));

  Ok(final_content)
}

pub fn inject(config: &Config) -> Result<(), String> {
  let p_ret = PathBuf::from_str(&shellexpand::tilde(&config.ios_dir));
  if let Err(err) = p_ret {
    return Err(err.to_string());
  }

  // TestUI.xcodeproj/project.pbxproj

  let ios_dir = p_ret.unwrap();
  let xcodeproj_name = format!("{}.xcodeproj", ios_dir.file_name().unwrap().to_str().unwrap());

  let project_pbxproj_path = ios_dir
    .join(xcodeproj_name)
    .join("project.pbxproj");

  println!(">>> project_pbxproj_path = {project_pbxproj_path:?}");
  match std::fs::read_to_string(&project_pbxproj_path) {
    Ok(content) => {
      let new_content = parse(config, content)?;
      println!(">>> new_content = {new_content}");
      if let Err(err) = std::fs::write(project_pbxproj_path, new_content) {
        return Err(err.to_string());
      }
      Ok(())
    },
    Err(err) => Err(err.to_string())
  }
}
