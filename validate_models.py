#!/usr/bin/env python3
"""
Network Topology Builder - glTF/GLB Model Validator
====================================================

This script validates .glb model files for use in the NTB application.
It checks:
- Material properties (colors, textures)
- Model dimensions and scale
- Selection radius compatibility (0.6 unit threshold)
- Bounding box calculations
- glTF specification compliance (via gltf-transform)

Usage:
    ./validate_models.py                    # Scan all models in public/models
    ./validate_models.py path/to/model.glb  # Validate single model
    ./validate_models.py --fix-scale        # Show Blender scaling recommendations
"""

import json
import struct
import sys
import os
import subprocess
from pathlib import Path
from typing import Dict, List, Optional, Tuple

# ANSI color codes for terminal output
class Colors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

# NTB application constants
NTB_SELECTION_RADIUS = 0.6  # Fixed selection radius in topology_viewport.rs:1230
NTB_IDEAL_SIZE = 1.0        # Ideal model size for good UX
NTB_MAX_SIZE = 1.5          # Maximum recommended size

class ModelIssue:
    """Represents a validation issue found in a model"""
    def __init__(self, severity: str, category: str, message: str, fix: Optional[str] = None):
        self.severity = severity  # 'error', 'warning', 'info'
        self.category = category  # 'material', 'scale', 'selection', 'structure'
        self.message = message
        self.fix = fix

class ModelValidationResult:
    """Results from validating a single model"""
    def __init__(self, file_path: str):
        self.file_path = file_path
        self.file_name = os.path.basename(file_path)
        self.file_size_mb = 0.0
        self.materials: List[Dict] = []
        self.bounding_boxes: List[Dict] = []
        self.max_dimension = 0.0
        self.issues: List[ModelIssue] = []
        self.gltf_transform_output: Optional[str] = None

    def add_issue(self, severity: str, category: str, message: str, fix: Optional[str] = None):
        self.issues.append(ModelIssue(severity, category, message, fix))

    def has_errors(self) -> bool:
        return any(i.severity == 'error' for i in self.issues)

    def has_warnings(self) -> bool:
        return any(i.severity == 'warning' for i in self.issues)

    def is_healthy(self) -> bool:
        return not self.has_errors() and not self.has_warnings()

def parse_glb_file(file_path: str) -> Optional[Dict]:
    """Parse GLB binary file and extract JSON metadata"""
    try:
        with open(file_path, 'rb') as f:
            # Read GLB header (12 bytes)
            magic = struct.unpack('I', f.read(4))[0]
            if magic != 0x46546C67:  # "glTF" in ASCII
                return None

            version = struct.unpack('I', f.read(4))[0]
            length = struct.unpack('I', f.read(4))[0]

            # Read JSON chunk header (8 bytes)
            chunk_length = struct.unpack('I', f.read(4))[0]
            chunk_type = struct.unpack('I', f.read(4))[0]

            # Read JSON data
            json_data = json.loads(f.read(chunk_length).decode('utf-8'))
            return json_data
    except Exception as e:
        print(f"{Colors.FAIL}Error parsing {file_path}: {e}{Colors.ENDC}")
        return None

def run_gltf_transform_inspect(file_path: str) -> Optional[str]:
    """Run gltf-transform inspect command and return output"""
    try:
        result = subprocess.run(
            ['gltf-transform', 'inspect', file_path],
            capture_output=True,
            text=True,
            timeout=30
        )
        if result.returncode == 0:
            return result.stdout
        return None
    except (subprocess.TimeoutExpired, FileNotFoundError, Exception):
        return None

def validate_model(file_path: str, show_gltf_transform: bool = True) -> ModelValidationResult:
    """Validate a single .glb model file"""
    result = ModelValidationResult(file_path)

    # Get file size
    result.file_size_mb = os.path.getsize(file_path) / (1024 * 1024)

    # Parse GLB file
    json_data = parse_glb_file(file_path)
    if not json_data:
        result.add_issue('error', 'structure', 'Failed to parse GLB file - may be corrupted')
        return result

    # Check materials
    if 'materials' not in json_data or len(json_data['materials']) == 0:
        result.add_issue('error', 'material', 'No materials found in model',
                        'In Blender: Ensure objects have materials assigned before export')
    else:
        for i, mat in enumerate(json_data['materials']):
            mat_name = mat.get('name', f'Material_{i}')
            mat_info = {'name': mat_name, 'index': i}

            has_color = False
            has_texture = False

            if 'pbrMetallicRoughness' in mat:
                pbr = mat['pbrMetallicRoughness']

                # Check base color factor
                if 'baseColorFactor' in pbr:
                    has_color = True
                    color = pbr['baseColorFactor']
                    mat_info['baseColor'] = f"RGBA({color[0]:.2f}, {color[1]:.2f}, {color[2]:.2f}, {color[3]:.2f})"

                # Check base color texture
                if 'baseColorTexture' in pbr:
                    has_texture = True
                    mat_info['hasTexture'] = True

                # Store metallic/roughness
                mat_info['metallic'] = pbr.get('metallicFactor', 1.0)
                mat_info['roughness'] = pbr.get('roughnessFactor', 1.0)

            result.materials.append(mat_info)

            # Validate material has color OR texture
            if not has_color and not has_texture:
                result.add_issue('error', 'material',
                               f'Material "{mat_name}" has no color or texture',
                               f'In Blender: Select material "{mat_name}" → Set Base Color in Principled BSDF')

    # Check bounding boxes and calculate max dimension
    if 'accessors' in json_data:
        max_dim = 0.0
        for i, acc in enumerate(json_data['accessors']):
            if 'min' in acc and 'max' in acc and len(acc['min']) == 3:
                min_v = acc['min']
                max_v = acc['max']
                size = [max_v[j] - min_v[j] for j in range(3)]
                bbox_max = max(size)
                max_dim = max(max_dim, bbox_max)

                result.bounding_boxes.append({
                    'accessor': i,
                    'min': min_v,
                    'max': max_v,
                    'size': size,
                    'max_dimension': bbox_max
                })

        result.max_dimension = max_dim

        # Check if model is too large for selection
        if max_dim > NTB_SELECTION_RADIUS * 2:
            scale_factor = (NTB_IDEAL_SIZE / max_dim) if max_dim > 0 else 1.0
            result.add_issue('error', 'selection',
                           f'Model too large ({max_dim:.2f} units) for selection radius ({NTB_SELECTION_RADIUS} units)',
                           f'In Blender: Select All (A) → Scale (S) → type {scale_factor:.3f} → Apply Scale (Ctrl+A)')
        elif max_dim > NTB_IDEAL_SIZE:
            scale_factor = (NTB_IDEAL_SIZE / max_dim) if max_dim > 0 else 1.0
            result.add_issue('warning', 'scale',
                           f'Model larger than ideal ({max_dim:.2f} units > {NTB_IDEAL_SIZE} units)',
                           f'Recommended: Scale by {scale_factor:.3f} in Blender for better UX')
        elif max_dim < 0.3:
            result.add_issue('warning', 'scale',
                           f'Model very small ({max_dim:.2f} units) - may be hard to see',
                           'Consider scaling up in Blender')

    # Run gltf-transform inspect if available
    if show_gltf_transform:
        output = run_gltf_transform_inspect(file_path)
        if output:
            result.gltf_transform_output = output

    return result

def print_validation_result(result: ModelValidationResult, verbose: bool = True):
    """Print formatted validation results"""
    # Header
    status_icon = "✓" if result.is_healthy() else ("⚠" if result.has_warnings() and not result.has_errors() else "✗")
    status_color = Colors.OKGREEN if result.is_healthy() else (Colors.WARNING if result.has_warnings() and not result.has_errors() else Colors.FAIL)

    print(f"\n{Colors.BOLD}{'='*80}{Colors.ENDC}")
    print(f"{status_color}{status_icon} {result.file_name}{Colors.ENDC} ({result.file_size_mb:.2f} MB)")
    print(f"{Colors.BOLD}{'='*80}{Colors.ENDC}")

    # Materials
    if result.materials:
        print(f"\n{Colors.BOLD}📦 Materials ({len(result.materials)}){Colors.ENDC}")
        for mat in result.materials:
            mat_status = Colors.OKGREEN + "✓" if 'baseColor' in mat or mat.get('hasTexture') else Colors.FAIL + "✗"
            print(f"  {mat_status} [{mat['index']}] {mat['name']}{Colors.ENDC}")
            if 'baseColor' in mat:
                print(f"      Color: {mat['baseColor']}")
            if mat.get('hasTexture'):
                print(f"      Texture: {Colors.OKGREEN}Yes{Colors.ENDC}")
            if 'metallic' in mat:
                print(f"      Metallic: {mat['metallic']:.2f} | Roughness: {mat['roughness']:.2f}")

    # Bounding boxes
    if result.bounding_boxes:
        print(f"\n{Colors.BOLD}📏 Bounding Boxes & Selection{Colors.ENDC}")

        # Color-code the max dimension based on size
        max_dim = result.max_dimension
        if max_dim > NTB_SELECTION_RADIUS * 2:
            size_status = Colors.FAIL + "✗ TOO LARGE"
            size_color = Colors.FAIL
        elif max_dim > NTB_IDEAL_SIZE:
            size_status = Colors.WARNING + "⚠ LARGER THAN IDEAL"
            size_color = Colors.WARNING
        elif max_dim < 0.3:
            size_status = Colors.WARNING + "⚠ VERY SMALL"
            size_color = Colors.WARNING
        else:
            size_status = Colors.OKGREEN + "✓ OPTIMAL SIZE"
            size_color = Colors.OKGREEN

        print(f"  Max Dimension: {size_color}{result.max_dimension:.3f} units{Colors.ENDC} {size_status}{Colors.ENDC}")
        print(f"  Ideal Range: {Colors.OKGREEN}0.5 - {NTB_IDEAL_SIZE} units{Colors.ENDC}")
        print(f"  Old Fixed Radius: {NTB_SELECTION_RADIUS} units (now auto-calculated from model)")

        # Show scaling recommendation
        if max_dim > NTB_IDEAL_SIZE:
            scale_factor = NTB_IDEAL_SIZE / max_dim
            print(f"  {Colors.OKCYAN}💡 Recommended Scale: {scale_factor:.3f}x (will make it {NTB_IDEAL_SIZE:.1f} units){Colors.ENDC}")
        elif max_dim < 0.3:
            scale_factor = 0.5 / max_dim
            print(f"  {Colors.OKCYAN}💡 Recommended Scale: {scale_factor:.3f}x (will make it 0.5 units){Colors.ENDC}")

        if verbose and len(result.bounding_boxes) <= 5:
            print(f"\n  {Colors.BOLD}Detailed Geometry:{Colors.ENDC}")
            for bbox in result.bounding_boxes:
                size = bbox['size']
                print(f"    Accessor {bbox['accessor']}: ({size[0]:.2f}, {size[1]:.2f}, {size[2]:.2f})")

    # Issues
    if result.issues:
        errors = [i for i in result.issues if i.severity == 'error']
        warnings = [i for i in result.issues if i.severity == 'warning']

        if errors:
            print(f"\n{Colors.FAIL}{Colors.BOLD}❌ Errors ({len(errors)}){Colors.ENDC}")
            for issue in errors:
                print(f"  {Colors.FAIL}• [{issue.category.upper()}] {issue.message}{Colors.ENDC}")
                if issue.fix:
                    print(f"    {Colors.OKCYAN}Fix: {issue.fix}{Colors.ENDC}")

        if warnings:
            print(f"\n{Colors.WARNING}{Colors.BOLD}⚠️  Warnings ({len(warnings)}){Colors.ENDC}")
            for issue in warnings:
                print(f"  {Colors.WARNING}• [{issue.category.upper()}] {issue.message}{Colors.ENDC}")
                if issue.fix:
                    print(f"    {Colors.OKCYAN}Fix: {issue.fix}{Colors.ENDC}")
    else:
        print(f"\n{Colors.OKGREEN}✓ No issues found - model is ready to use!{Colors.ENDC}")

    # gltf-transform output
    if verbose and result.gltf_transform_output:
        print(f"\n{Colors.BOLD}🔍 gltf-transform inspect{Colors.ENDC}")
        print(result.gltf_transform_output)

def find_all_models(base_path: str = "public/models") -> List[str]:
    """Find all .glb files in the models directory"""
    models = []
    for root, dirs, files in os.walk(base_path):
        for file in files:
            if file.endswith('.glb'):
                models.append(os.path.join(root, file))
    return sorted(models)

def print_summary(results: List[ModelValidationResult]):
    """Print summary of all validation results"""
    total = len(results)
    healthy = sum(1 for r in results if r.is_healthy())
    warnings_only = sum(1 for r in results if r.has_warnings() and not r.has_errors())
    errors = sum(1 for r in results if r.has_errors())

    print(f"\n{Colors.BOLD}{'='*80}{Colors.ENDC}")
    print(f"{Colors.BOLD}📊 VALIDATION SUMMARY{Colors.ENDC}")
    print(f"{Colors.BOLD}{'='*80}{Colors.ENDC}")
    print(f"Total models: {total}")
    print(f"{Colors.OKGREEN}✓ Healthy: {healthy}{Colors.ENDC}")
    print(f"{Colors.WARNING}⚠ Warnings only: {warnings_only}{Colors.ENDC}")
    print(f"{Colors.FAIL}✗ Errors: {errors}{Colors.ENDC}")

    if errors > 0:
        print(f"\n{Colors.FAIL}{Colors.BOLD}Models requiring fixes:{Colors.ENDC}")
        for r in results:
            if r.has_errors():
                error_count = sum(1 for i in r.issues if i.severity == 'error')
                print(f"  • {r.file_name} ({error_count} error{'s' if error_count > 1 else ''})")

def main():
    """Main entry point"""
    import argparse

    parser = argparse.ArgumentParser(
        description='Validate glTF/GLB models for Network Topology Builder',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog='''
Examples:
  %(prog)s                                    # Scan all models
  %(prog)s public/models/router/cisco/router.glb  # Single model
  %(prog)s --no-gltf-transform                # Skip gltf-transform output
  %(prog)s --summary-only                     # Show only summary
        '''
    )
    parser.add_argument('files', nargs='*', help='Specific .glb files to validate')
    parser.add_argument('--no-gltf-transform', action='store_true',
                       help='Skip gltf-transform inspection output')
    parser.add_argument('--summary-only', action='store_true',
                       help='Show only summary, not individual results')
    parser.add_argument('-v', '--verbose', action='store_true',
                       help='Show detailed output')

    args = parser.parse_args()

    # Determine which files to validate
    if args.files:
        model_files = args.files
    else:
        print(f"{Colors.BOLD}🔍 Scanning for models in public/models...{Colors.ENDC}")
        model_files = find_all_models()
        if not model_files:
            print(f"{Colors.FAIL}No .glb files found in public/models{Colors.ENDC}")
            return 1
        print(f"Found {len(model_files)} model(s)\n")

    # Validate each model
    results = []
    for file_path in model_files:
        if not os.path.exists(file_path):
            print(f"{Colors.FAIL}File not found: {file_path}{Colors.ENDC}")
            continue

        result = validate_model(file_path, show_gltf_transform=not args.no_gltf_transform)
        results.append(result)

        if not args.summary_only:
            print_validation_result(result, verbose=args.verbose)

    # Print summary
    if len(results) > 1:
        print_summary(results)

    # Exit code based on validation results
    if any(r.has_errors() for r in results):
        return 1
    return 0

if __name__ == '__main__':
    sys.exit(main())
