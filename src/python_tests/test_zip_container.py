import unittest
from zip_container import ZipContainer

class TestZipContainer(unittest.TestCase):

    def setUp(self):
        # Initialize a ZipContainer instance for testing
        self.zip_container = ZipContainer("https://raw.githubusercontent.com/holg/gldf-rs/refs/heads/master/tests/data/test.gldf", "product.xml")

    def test_zip_data(self):
        # Test the zip_data getter
        zip_data = self.zip_container.zip_data
        self.assertIsNotNone(zip_data, "Zip data should not be None")

    def test_load_definition_file_str(self):
        # Test loading the definition file as a string
        definition_str = self.zip_container.load_definition_file_str # it is a getter
        self.assertIsInstance(definition_str, str, "Definition file should be a string")

    def test_get_zip_files(self):
        # Test retrieving files from the ZIP
        files = self.zip_container.get_zip_files()
        self.assertIsInstance(files, list, "Files should be returned as a list")
        self.assertGreater(len(files), 0, "There should be at least one file in the ZIP")

if __name__ == '__main__':
    unittest.main()