{ lib }:

let
  # Helper function to parse a version string into components
  parseVersion =
    version:
    let
      version' = lib.strings.trim version;

      # Handle wildcard version
      isWildcard = version' == "*";

      # Regular expression to match version patterns
      # Captures: prefix (^, >=), major, minor, patch, and pre-release
      matches = builtins.match "^(\\^|>=)?(([0-9]+)|x)\\.((([0-9]+)|x))\\.((([0-9]+)|x))(\\-.*)?$" version';

      # Extract components if matches found
      prefix = if matches != null then builtins.elemAt matches 0 else null;
      majorStr = if matches != null then builtins.elemAt matches 1 else null;
      minorStr = if matches != null then builtins.elemAt matches 3 else null;
      patchStr = if matches != null then builtins.elemAt matches 7 else null;
      preRelease = if matches != null then builtins.elemAt matches 10 else null;

      # Convert to appropriate types with defaults
      hasCaret = prefix == "^";
      hasGreaterEquals = prefix == ">=";

      # Helper function for conditional expressions (similar to texpr macro)
      texpr =
        condition: ifTrue: ifFalse:
        if condition then ifTrue else ifFalse;

      majorBase = texpr (majorStr == "x") 0 (lib.strings.toInt majorStr);
      majorMustEqual = texpr (majorStr == "x") false true;

      minorBase = texpr (minorStr == "x") 0 (lib.strings.toInt minorStr);
      minorMustEqual = texpr (minorStr == "x") false true;

      patchBase = texpr (patchStr == "x") 0 (lib.strings.toInt patchStr);
      patchMustEqual = texpr (patchStr == "x") false true;
    in
    if isWildcard then
      {
        hasCaret = false;
        hasGreaterEquals = false;
        majorBase = 0;
        majorMustEqual = false;
        minorBase = 0;
        minorMustEqual = false;
        patchBase = 0;
        patchMustEqual = false;
        preRelease = null;
      }
    else if matches == null then
      null
    else
      {
        inherit
          hasCaret
          hasGreaterEquals
          majorBase
          majorMustEqual
          minorBase
          minorMustEqual
          patchBase
          patchMustEqual
          preRelease
          ;
      };

  # Normalize a parsed version
  normalizeVersion =
    parsedVersion:
    let
      majorBase = parsedVersion.majorBase;
      majorMustEqual = parsedVersion.majorMustEqual;
      minorBase = parsedVersion.minorBase;
      patchBase = parsedVersion.patchBase;

      # Apply caret rules
      minorMustEqual' =
        if parsedVersion.hasCaret then
          if majorBase == 0 then parsedVersion.minorMustEqual else false
        else
          parsedVersion.minorMustEqual;

      patchMustEqual' =
        if parsedVersion.hasCaret then
          if majorBase == 0 then false else false
        else
          parsedVersion.patchMustEqual;

      # We're not implementing the date-based pre-release handling for simplicity
      notBefore = 0;
    in
    {
      majorBase = majorBase;
      majorMustEqual = majorMustEqual;
      minorBase = minorBase;
      minorMustEqual = minorMustEqual';
      patchBase = patchBase;
      patchMustEqual = patchMustEqual';
      isMinimum = parsedVersion.hasGreaterEquals;
      notBefore = notBefore;
    };

  # Check if a version is valid against a desired version
  isValidVersion =
    version: desiredVersion:
    let
      majorBase = version.majorBase;
      minorBase = version.minorBase;
      patchBase = version.patchBase;

      desiredMajorBase = desiredVersion.majorBase;
      desiredMinorBase = desiredVersion.minorBase;
      desiredPatchBase = desiredVersion.patchBase;

      majorMustEqual = desiredVersion.majorMustEqual;
      minorMustEqual = desiredVersion.minorMustEqual;
      patchMustEqual = desiredVersion.patchMustEqual;

      # Handle minimum version case (>=)
      isMinimumVersion = desiredVersion.isMinimum;

      # Special case for 1.0.0 compatibility with 0.x.x
      specialCase =
        majorBase == 1 && desiredMajorBase == 0 && (!majorMustEqual || !minorMustEqual || !patchMustEqual);

      # Adjusted desired version for special case
      adjustedDesired =
        if specialCase then
          {
            majorBase = 1;
            majorMustEqual = true;
            minorBase = 0;
            minorMustEqual = false;
            patchBase = 0;
            patchMustEqual = false;
            isMinimum = desiredVersion.isMinimum;
            notBefore = desiredVersion.notBefore;
          }
        else
          desiredVersion;

      # Re-extract values after potential adjustment
      desiredMajorBase' = adjustedDesired.majorBase;
      desiredMinorBase' = adjustedDesired.minorBase;
      desiredPatchBase' = adjustedDesired.patchBase;
      majorMustEqual' = adjustedDesired.majorMustEqual;
      minorMustEqual' = adjustedDesired.minorMustEqual;
      patchMustEqual' = adjustedDesired.patchMustEqual;
    in
    # Handle minimum version case
    if isMinimumVersion then
      if majorBase > desiredMajorBase' then
        true
      else if majorBase < desiredMajorBase' then
        false
      else if minorBase > desiredMinorBase' then
        true
      else if minorBase < desiredMinorBase' then
        false
      else
        patchBase >= desiredPatchBase'

    # Regular version comparison
    else if majorBase < desiredMajorBase' then
      false
    else if majorBase > desiredMajorBase' then
      !majorMustEqual'
    else if minorBase < desiredMinorBase' then
      false
    else if minorBase > desiredMinorBase' then
      !minorMustEqual'
    else if patchBase < desiredPatchBase' then
      false
    else if patchBase > desiredPatchBase' then
      !patchMustEqual'
    else
      true;

in
{
  # Main function to check if a version is valid against a requested version
  isVersionValid =
    codeVersion: requestedVersion:
    let
      # Handle wildcard version
      isWildcard = lib.strings.trim requestedVersion == "*";

      # Parse and normalize the requested version
      parsedRequestedVersion = parseVersion requestedVersion;

      # Return true for wildcard version
      result =
        if isWildcard then
          true
        # Return false if parsing failed
        else if parsedRequestedVersion == null then
          false
        else
          let
            normalizedRequestedVersion = normalizeVersion parsedRequestedVersion;

            # Special validation for 0.x.x versions
            isValid0x =
              if normalizedRequestedVersion.majorBase == 0 then
                if !normalizedRequestedVersion.majorMustEqual || !normalizedRequestedVersion.minorMustEqual then
                  false
                else
                  true
              else if !normalizedRequestedVersion.majorMustEqual then
                false
              else
                true;
          in
          if !isValid0x then
            false
          else
            let
              # Parse and normalize the code version
              parsedCodeVersion = parseVersion codeVersion;
            in
            if parsedCodeVersion == null then
              false
            else
              let
                normalizedCodeVersion = normalizeVersion parsedCodeVersion;
              in
              isValidVersion normalizedCodeVersion normalizedRequestedVersion;
    in
    result;
}
