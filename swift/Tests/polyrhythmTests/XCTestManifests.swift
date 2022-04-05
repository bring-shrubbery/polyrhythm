import XCTest

#if !canImport(ObjectiveC)
public func allTests() -> [XCTestCaseEntry] {
    return [
        testCase(polyrhythm_swiftTests.allTests),
    ]
}
#endif
